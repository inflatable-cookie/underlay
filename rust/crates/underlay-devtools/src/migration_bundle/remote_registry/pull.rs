use reqwest::header::ACCEPT;

use super::super::{
    decode_package, sha256_digest, validate_bundle_package, write_pulled_outputs,
    BundlePullOptions, BundlePullReport, MigrationBundleError,
};
use super::client::{ping_registry, registry_client};
use super::reference::{is_digest_reference, parse_remote_ref};

pub(in crate::migration_bundle) fn remote_pull(
    options: &BundlePullOptions,
) -> Result<BundlePullReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(&options.oci_ref)?;
    let client = registry_client()?;

    ping_registry(&client, &remote_ref.registry)?;

    let manifest_url = format!(
        "{}/v2/{}/manifests/{}",
        remote_ref.registry, remote_ref.repository, remote_ref.reference
    );
    let response = client
        .get(&manifest_url)
        .header(
            ACCEPT,
            "application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json",
        )
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry manifest fetch failed: status={}",
            response.status()
        )));
    }

    let manifest_digest = response
        .headers()
        .get("docker-content-digest")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or_default();

    let manifest: serde_json::Value = response
        .json()
        .map_err(|err| MigrationBundleError::Validation(format!("invalid manifest JSON: {err}")))?;

    let layer_digest = manifest
        .get("layers")
        .and_then(|v| v.as_array())
        .and_then(|layers| layers.first())
        .and_then(|layer| layer.get("digest"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            MigrationBundleError::Validation("manifest missing primary package layer".to_string())
        })?
        .to_string();

    let blob_url = format!(
        "{}/v2/{}/blobs/{}",
        remote_ref.registry, remote_ref.repository, layer_digest
    );
    let blob_response = client
        .get(&blob_url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    if !blob_response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry blob fetch failed: status={}",
            blob_response.status()
        )));
    }
    let bytes = blob_response
        .bytes()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
        .to_vec();

    let actual_layer_digest = sha256_digest(&bytes);
    if actual_layer_digest != layer_digest {
        return Err(MigrationBundleError::Validation(format!(
            "remote blob digest mismatch: expected {}, found {}",
            layer_digest, actual_layer_digest
        )));
    }

    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;
    let output_file = write_pulled_outputs(&package, &options.output_dir)?;

    let digest = if is_digest_reference(&manifest_digest) {
        manifest_digest
    } else {
        layer_digest
    };

    Ok(BundlePullReport {
        oci_ref: options.oci_ref.clone(),
        output_file,
        digest,
        status: "pulled-remote".to_string(),
    })
}
