use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_media::version_key;

use super::{sha256_digest, MigrationBundleError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MediaKeyMapping {
    pub media_id: String,
    pub version_id: String,
    pub object_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MediaAssetPayload {
    pub relative_path: String,
    pub filename: String,
    pub byte_size: u64,
    pub sha256: String,
    pub content_base64: String,
    pub mapping: MediaKeyMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) struct MediaShardPayload {
    schema_version: String,
    shard_id: String,
    assets: Vec<MediaAssetPayload>,
}

impl MediaShardPayload {
    pub(super) fn shard_id(&self) -> &str {
        &self.shard_id
    }

    pub(super) fn asset_count(&self) -> usize {
        self.assets.len()
    }
}

#[derive(Debug, Clone)]
pub(super) struct RawMediaEntry {
    relative_path: String,
    filename: String,
    bytes: Vec<u8>,
    sha256: String,
}

pub(super) fn validate_media_shard_payload(bytes: &[u8]) -> Result<(), MigrationBundleError> {
    let shard: MediaShardPayload = serde_json::from_slice(bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid media shard payload JSON: {err}"))
    })?;

    for asset in shard.assets {
        let content = BASE64.decode(asset.content_base64).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid base64 content for media asset {}: {err}",
                asset.relative_path
            ))
        })?;

        if content.len() as u64 != asset.byte_size {
            return Err(MigrationBundleError::Validation(format!(
                "media asset size mismatch for {}: expected {}, found {}",
                asset.relative_path,
                asset.byte_size,
                content.len()
            )));
        }

        let actual = sha256_digest(&content);
        if actual != asset.sha256 {
            return Err(MigrationBundleError::Validation(format!(
                "media asset digest mismatch for {}: expected {}, found {}",
                asset.relative_path, asset.sha256, actual
            )));
        }

        let media_id = uuid::Uuid::parse_str(&asset.mapping.media_id).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid mapping media_id for {}: {err}",
                asset.relative_path
            ))
        })?;
        let version_id = uuid::Uuid::parse_str(&asset.mapping.version_id).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid mapping version_id for {}: {err}",
                asset.relative_path
            ))
        })?;

        let expected_key = version_key(media_id, version_id, &asset.filename);
        if expected_key != asset.mapping.object_key {
            return Err(MigrationBundleError::Validation(format!(
                "mapping object_key mismatch for {}: expected {}, found {}",
                asset.relative_path, expected_key, asset.mapping.object_key
            )));
        }
    }

    Ok(())
}

pub(super) fn build_media_shards(
    entries: &[RawMediaEntry],
    shard_max_bytes: u64,
) -> Result<Vec<MediaShardPayload>, MigrationBundleError> {
    let mut shards = Vec::new();

    if entries.is_empty() {
        shards.push(MediaShardPayload {
            schema_version: "1".to_string(),
            shard_id: "media-0001".to_string(),
            assets: Vec::new(),
        });
        return Ok(shards);
    }

    let mut current_assets: Vec<MediaAssetPayload> = Vec::new();
    let mut current_bytes: u64 = 0;
    let mut shard_index: u64 = 1;

    for entry in entries {
        let byte_size = entry.bytes.len() as u64;
        let should_rotate =
            !current_assets.is_empty() && current_bytes + byte_size > shard_max_bytes;

        if should_rotate {
            shards.push(MediaShardPayload {
                schema_version: "1".to_string(),
                shard_id: format!("media-{shard_index:04}"),
                assets: current_assets,
            });
            shard_index += 1;
            current_assets = Vec::new();
            current_bytes = 0;
        }

        let media_uuid = deterministic_uuid_from_seed(&format!("media:{}", entry.sha256));
        let version_uuid = deterministic_uuid_from_seed(&format!("version:{}", entry.sha256));

        current_assets.push(MediaAssetPayload {
            relative_path: entry.relative_path.clone(),
            filename: entry.filename.clone(),
            byte_size,
            sha256: entry.sha256.clone(),
            content_base64: BASE64.encode(&entry.bytes),
            mapping: MediaKeyMapping {
                media_id: media_uuid.to_string(),
                version_id: version_uuid.to_string(),
                object_key: version_key(media_uuid, version_uuid, &entry.filename),
            },
        });
        current_bytes += byte_size;
    }

    shards.push(MediaShardPayload {
        schema_version: "1".to_string(),
        shard_id: format!("media-{shard_index:04}"),
        assets: current_assets,
    });

    Ok(shards)
}

pub(super) fn collect_media_entries(
    media_dir: Option<&PathBuf>,
) -> Result<Vec<RawMediaEntry>, MigrationBundleError> {
    let mut entries = Vec::new();

    if let Some(media_dir) = media_dir {
        if !media_dir.exists() {
            return Err(MigrationBundleError::InvalidInput(format!(
                "media_dir does not exist: {}",
                media_dir.display()
            )));
        }

        for file in collect_files_recursive(media_dir)? {
            let rel = file.strip_prefix(media_dir).map_err(|err| {
                MigrationBundleError::Validation(format!(
                    "failed to strip media_dir prefix for {}: {err}",
                    file.display()
                ))
            })?;
            let bytes = std::fs::read(&file)?;
            let filename = file
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| {
                    MigrationBundleError::Validation(format!(
                        "media filename is not valid UTF-8: {}",
                        file.display()
                    ))
                })?
                .to_string();

            entries.push(RawMediaEntry {
                relative_path: rel.to_string_lossy().to_string(),
                filename,
                sha256: sha256_digest(&bytes),
                bytes,
            });
        }
    }

    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(entries)
}

fn deterministic_uuid_from_seed(seed: &str) -> uuid::Uuid {
    let hash = Sha256::digest(seed.as_bytes());
    let mut bytes: [u8; 16] = hash[..16]
        .try_into()
        .expect("sha256 prefix should have 16 bytes");

    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    uuid::Uuid::from_bytes(bytes)
}

fn collect_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, MigrationBundleError> {
    let mut files = Vec::new();
    visit_dir(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), MigrationBundleError> {
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dir(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}
