use std::path::{Path, PathBuf};

use chrono::Utc;

use super::{
    decode_package, sha256_digest, validate_bundle_package, write_pulled_outputs,
    BundlePublishOptions, BundlePublishReport, BundlePullOptions, BundlePullReport,
    MigrationBundleError, SHA256_PREFIX,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalStoreDir {
    path: PathBuf,
}

impl LocalStoreDir {
    pub(crate) fn resolve(local_store_dir: Option<&PathBuf>) -> Result<Self, MigrationBundleError> {
        let path = if let Some(dir) = local_store_dir {
            dir.clone()
        } else if let Ok(env_dir) = std::env::var("UNDERLAY_LOCAL_OCI_DIR") {
            PathBuf::from(env_dir)
        } else {
            PathBuf::from(".underlay-local-oci")
        };

        if path.as_os_str().is_empty() {
            return Err(MigrationBundleError::InvalidInput(
                "local_store_dir must not be empty".to_string(),
            ));
        }

        Ok(Self { path })
    }

    pub(crate) fn as_path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn ref_path(&self, oci_ref: &str) -> PathBuf {
        self.path
            .join("refs")
            .join(format!("{}.json", sanitize_ref(oci_ref)))
    }

    pub(crate) fn blob_path_for_digest(
        &self,
        digest: &str,
    ) -> Result<PathBuf, MigrationBundleError> {
        validate_sha256_digest(digest)?;
        let digest_hex = digest.strip_prefix(SHA256_PREFIX).unwrap_or(digest);
        Ok(self
            .path
            .join("blobs")
            .join("sha256")
            .join(format!("{digest_hex}.json")))
    }
}

pub(super) fn publish_local_bundle(
    options: &BundlePublishOptions,
    bytes: &[u8],
    digest: String,
) -> Result<BundlePublishReport, MigrationBundleError> {
    validate_sha256_digest(&digest)?;
    let store = resolve_local_store_dir(options.local_store_dir.as_ref())?;
    let blob_path = store.blob_path_for_digest(&digest)?;
    if let Some(parent) = blob_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&blob_path, bytes)?;

    let ref_dir = store.as_path().join("refs");
    std::fs::create_dir_all(&ref_dir)?;
    let ref_path = store.ref_path(&options.oci_ref);
    let ref_payload = serde_json::to_vec_pretty(&serde_json::json!({
        "oci_ref": options.oci_ref,
        "digest": digest,
        "blob_path": blob_path,
        "published_at": Utc::now(),
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(ref_path, ref_payload)?;

    Ok(BundlePublishReport {
        bundle_file: options.bundle_file.clone(),
        oci_ref: options.oci_ref.clone(),
        digest,
        status: "published-local".to_string(),
    })
}

pub(super) fn pull_local_bundle(
    options: &BundlePullOptions,
) -> Result<BundlePullReport, MigrationBundleError> {
    let store = resolve_local_store_dir(options.local_store_dir.as_ref())?;
    let digest = match digest_from_ref(&options.oci_ref)? {
        Some(digest) => digest,
        None => resolve_ref_digest(&store, &options.oci_ref)?,
    };

    let blob_path = store.blob_path_for_digest(&digest)?;
    if !blob_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "bundle blob not found for digest {}",
            digest
        )));
    }

    let bytes = std::fs::read(&blob_path)?;
    let actual_digest = sha256_digest(&bytes);
    if actual_digest != digest {
        return Err(MigrationBundleError::Validation(format!(
            "bundle blob digest mismatch: expected {}, found {}",
            digest, actual_digest
        )));
    }

    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;
    let output_file = write_pulled_outputs(&package, &options.output_dir)?;

    Ok(BundlePullReport {
        oci_ref: options.oci_ref.clone(),
        output_file,
        digest,
        status: "pulled-local".to_string(),
    })
}

pub(crate) fn resolve_local_store_dir(
    local_store_dir: Option<&PathBuf>,
) -> Result<LocalStoreDir, MigrationBundleError> {
    LocalStoreDir::resolve(local_store_dir)
}

pub(crate) fn sanitize_ref(oci_ref: &str) -> String {
    oci_ref
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

pub(crate) fn digest_from_ref(oci_ref: &str) -> Result<Option<String>, MigrationBundleError> {
    let Some((_, digest)) = oci_ref.split_once('@') else {
        return Ok(None);
    };
    if !digest.starts_with(SHA256_PREFIX) {
        return Ok(None);
    }
    validate_sha256_digest(digest)?;
    Ok(Some(digest.to_string()))
}

pub(crate) fn resolve_digest(
    store: &LocalStoreDir,
    oci_ref: &str,
) -> Result<String, MigrationBundleError> {
    if let Some(digest) = digest_from_ref(oci_ref)? {
        return Ok(digest);
    }
    resolve_ref_digest(store, oci_ref)
}

fn resolve_ref_digest(
    store: &LocalStoreDir,
    oci_ref: &str,
) -> Result<String, MigrationBundleError> {
    let ref_path = store.ref_path(oci_ref);
    if !ref_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "oci_ref not found in local store: {}",
            oci_ref
        )));
    }

    let bytes = std::fs::read(&ref_path)?;
    let payload: serde_json::Value = serde_json::from_slice(&bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid ref mapping JSON: {err}"))
    })?;

    let digest = payload
        .get("digest")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            MigrationBundleError::Validation(format!(
                "ref mapping missing digest field: {}",
                ref_path.display()
            ))
        })?;

    validate_sha256_digest(digest)?;
    Ok(digest.to_string())
}

pub(crate) fn validate_sha256_digest(digest: &str) -> Result<(), MigrationBundleError> {
    let Some(hex) = digest.strip_prefix(SHA256_PREFIX) else {
        return Err(MigrationBundleError::InvalidInput(format!(
            "digest must start with {SHA256_PREFIX}"
        )));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(MigrationBundleError::InvalidInput(
            "sha256 digest must contain 64 hex characters".to_string(),
        ));
    }
    Ok(())
}
