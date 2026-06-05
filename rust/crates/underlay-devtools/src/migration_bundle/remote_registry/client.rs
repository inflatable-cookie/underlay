use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, LOCATION};

use super::super::MigrationBundleError;

pub(super) fn registry_client() -> Result<Client, MigrationBundleError> {
    Client::builder()
        .build()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))
}

pub(super) fn ping_registry(client: &Client, registry: &str) -> Result<(), MigrationBundleError> {
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

pub(super) fn upload_blob(
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
