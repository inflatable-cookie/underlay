use reqwest::header::CONTENT_TYPE;

use super::super::{
    decode_package, sha256_digest, BundlePublishOptions, BundlePublishReport, MigrationBundleError,
    OCI_MANIFEST_MEDIA_TYPE,
};
use super::client::{ping_registry, registry_client, upload_blob};
use super::reference::{is_digest_reference, parse_remote_ref};
use super::{OCI_CONFIG_MEDIA_TYPE, OCI_PACKAGE_LAYER_MEDIA_TYPE};

pub(in crate::migration_bundle) fn remote_publish(
    options: &BundlePublishOptions,
    package_bytes: &[u8],
) -> Result<BundlePublishReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(options.oci_ref())?;
    if is_digest_reference(&remote_ref.reference) {
        return Err(MigrationBundleError::InvalidInput(
            "remote publish requires a tag reference, not digest".to_string(),
        ));
    }

    let package_digest = sha256_digest(package_bytes);
    let package = decode_package(package_bytes)?;
    let config_bytes = serde_json::to_vec(&package.layout.config)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    let config_digest = sha256_digest(&config_bytes);

    let client = registry_client()?;
    ping_registry(&client, &remote_ref.registry)?;
    upload_blob(
        &client,
        &remote_ref.registry,
        &remote_ref.repository,
        &config_digest,
        &config_bytes,
    )?;
    upload_blob(
        &client,
        &remote_ref.registry,
        &remote_ref.repository,
        &package_digest,
        package_bytes,
    )?;

    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": OCI_MANIFEST_MEDIA_TYPE,
        "config": {
            "mediaType": OCI_CONFIG_MEDIA_TYPE,
            "digest": config_digest,
            "size": config_bytes.len()
        },
        "layers": [{
            "mediaType": OCI_PACKAGE_LAYER_MEDIA_TYPE,
            "digest": package_digest,
            "size": package_bytes.len()
        }]
    });
    let manifest_bytes = serde_json::to_vec(&manifest)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let manifest_url = format!(
        "{}/v2/{}/manifests/{}",
        remote_ref.registry, remote_ref.repository, remote_ref.reference
    );

    let response = client
        .put(&manifest_url)
        .header(CONTENT_TYPE, OCI_MANIFEST_MEDIA_TYPE)
        .body(manifest_bytes)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry manifest publish failed: status={}",
            response.status()
        )));
    }

    let manifest_digest = response
        .headers()
        .get("docker-content-digest")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or_default();

    let digest = if is_digest_reference(&manifest_digest) {
        manifest_digest
    } else {
        package_digest
    };

    Ok(BundlePublishReport {
        bundle_file: options.bundle_file().clone(),
        oci_ref: options.oci_ref().to_string(),
        digest,
        status: "published-remote".to_string(),
    })
}
