use super::super::{local_store, MigrationBundleError, SHA256_PREFIX};

#[derive(Debug, Clone)]
pub(super) struct RemoteRegistryRef {
    pub(super) registry: String,
    pub(super) repository: String,
    pub(super) reference: String,
}

pub(in crate::migration_bundle) fn is_remote_ref(oci_ref: &str) -> bool {
    oci_ref.starts_with("http://") || oci_ref.starts_with("https://")
}

pub(super) fn parse_remote_ref(input: &str) -> Result<RemoteRegistryRef, MigrationBundleError> {
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

pub(super) fn is_digest_reference(reference: &str) -> bool {
    reference.starts_with(SHA256_PREFIX)
}
