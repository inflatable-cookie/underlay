use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, LOCATION};

use super::{
    decode_package, local_store, sha256_digest, validate_bundle_package, write_pulled_outputs,
    BundlePublishOptions, BundlePublishReport, BundlePullOptions, BundlePullReport,
    MigrationBundleError, OCI_MANIFEST_MEDIA_TYPE, SHA256_PREFIX,
};

const OCI_CONFIG_MEDIA_TYPE: &str = "application/vnd.underlay.migration.bundle.config.v1+json";
const OCI_PACKAGE_LAYER_MEDIA_TYPE: &str = "application/vnd.underlay.bundle.package.v1+json";

#[derive(Debug, Clone)]
struct RemoteRegistryRef {
    registry: String,
    repository: String,
    reference: String,
}

pub(super) fn is_remote_ref(oci_ref: &str) -> bool {
    oci_ref.starts_with("http://") || oci_ref.starts_with("https://")
}

pub(super) fn remote_publish(
    options: &BundlePublishOptions,
    package_bytes: &[u8],
) -> Result<BundlePublishReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(&options.oci_ref)?;
    if remote_ref.reference.starts_with(SHA256_PREFIX) {
        return Err(MigrationBundleError::InvalidInput(
            "remote publish requires a tag reference, not digest".to_string(),
        ));
    }

    let package_digest = sha256_digest(package_bytes);
    let package = decode_package(package_bytes)?;
    let config_bytes = serde_json::to_vec(&package.layout.config)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    let config_digest = sha256_digest(&config_bytes);

    let client = Client::builder()
        .build()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

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

    let digest = if manifest_digest.starts_with(SHA256_PREFIX) {
        manifest_digest
    } else {
        package_digest
    };

    Ok(BundlePublishReport {
        bundle_file: options.bundle_file.clone(),
        oci_ref: options.oci_ref.clone(),
        digest,
        status: "published-remote".to_string(),
    })
}

pub(super) fn remote_pull(
    options: &BundlePullOptions,
) -> Result<BundlePullReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(&options.oci_ref)?;
    let client = Client::builder()
        .build()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

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

    let digest = if manifest_digest.starts_with(SHA256_PREFIX) {
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

fn parse_remote_ref(input: &str) -> Result<RemoteRegistryRef, MigrationBundleError> {
    let url = reqwest::Url::parse(input).map_err(|err| {
        MigrationBundleError::InvalidInput(format!("invalid remote oci_ref URL: {err}"))
    })?;
    let registry = format!(
        "{}://{}",
        url.scheme(),
        url.host_str().ok_or_else(|| {
            MigrationBundleError::InvalidInput("remote oci_ref missing host".to_string())
        })?
    );
    let registry = if let Some(port) = url.port() {
        format!("{registry}:{port}")
    } else {
        registry
    };

    let path = url.path().trim_start_matches('/');
    if path.is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "remote oci_ref must include repository and reference".to_string(),
        ));
    }

    if let Some((repository, digest)) = path.split_once('@') {
        if repository.is_empty() || digest.is_empty() {
            return Err(MigrationBundleError::InvalidInput(
                "remote oci_ref digest form must be <repo>@sha256:...".to_string(),
            ));
        }
        local_store::validate_sha256_digest(digest)?;
        return Ok(RemoteRegistryRef {
            registry,
            repository: repository.to_string(),
            reference: digest.to_string(),
        });
    }

    let slash = path.rfind('/').unwrap_or(0);
    let colon = path.rfind(':').ok_or_else(|| {
        MigrationBundleError::InvalidInput(
            "remote oci_ref tag form must be <repo>:<tag>".to_string(),
        )
    })?;
    if colon <= slash || colon == path.len() - 1 {
        return Err(MigrationBundleError::InvalidInput(
            "remote oci_ref tag form must be <repo>:<tag>".to_string(),
        ));
    }

    Ok(RemoteRegistryRef {
        registry,
        repository: path[..colon].to_string(),
        reference: path[colon + 1..].to_string(),
    })
}

fn ping_registry(client: &Client, registry: &str) -> Result<(), MigrationBundleError> {
    let url = format!("{registry}/v2/");
    let response = client
        .get(url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry ping failed: status={}",
            response.status()
        )));
    }
    Ok(())
}

fn upload_blob(
    client: &Client,
    registry: &str,
    repository: &str,
    digest: &str,
    bytes: &[u8],
) -> Result<(), MigrationBundleError> {
    let start_url = format!("{registry}/v2/{repository}/blobs/uploads/");
    let start = client
        .post(&start_url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if start.status() != reqwest::StatusCode::ACCEPTED {
        return Err(MigrationBundleError::Validation(format!(
            "blob upload start failed: status={}",
            start.status()
        )));
    }

    let location = start
        .headers()
        .get(LOCATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            MigrationBundleError::Validation("registry missing upload location".to_string())
        })?;

    let mut upload_url = if location.starts_with("http://") || location.starts_with("https://") {
        reqwest::Url::parse(location)
            .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
    } else {
        reqwest::Url::parse(&format!("{registry}{location}"))
            .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
    };
    upload_url.query_pairs_mut().append_pair("digest", digest);

    let finish = client
        .put(upload_url)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(bytes.to_vec())
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !finish.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "blob upload finish failed for {}: status={}",
            digest,
            finish.status()
        )));
    }

    Ok(())
}
