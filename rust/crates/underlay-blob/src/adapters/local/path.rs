use std::path::{Path, PathBuf};

use tokio::fs;

use crate::error::{BlobError, BlobResult};
use crate::types::{validate_blob_object_key, BlobObjectKeyError};

pub(super) fn validate_local_object_key(key: &str) -> BlobResult<()> {
    validate_blob_object_key(key).map_err(|err| {
        BlobError::InvalidKey(match err {
            BlobObjectKeyError::Empty => "key must not be empty".to_string(),
            BlobObjectKeyError::Absolute => "key must be a relative object path".to_string(),
            BlobObjectKeyError::InvalidCharacter => "key contains an invalid character".to_string(),
            BlobObjectKeyError::TraversalComponent => {
                "key must not contain traversal components".to_string()
            }
            BlobObjectKeyError::InvalidComponent => {
                "key contains an invalid path component".to_string()
            }
        })
    })
}

/// Check if a path is safely within the base directory.
///
/// Returns the canonicalized path if valid, None otherwise.
/// This resolves symlinks and `..` to prevent path traversal attacks.
pub(super) fn path_within_base(path: &Path, canonical_base: &Path) -> Option<PathBuf> {
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return None,
    };

    if canonical.starts_with(canonical_base) && canonical != canonical_base {
        Some(canonical)
    } else {
        None
    }
}

/// Clean up empty parent directories after file deletion.
///
/// Walks up from the deleted file's parent directory, removing empty
/// directories until reaching the base path or a non-empty directory.
pub(super) async fn cleanup_empty_parents(deleted_path: &Path, canonical_base: &Path) {
    let mut current = deleted_path.parent().map(|p| p.to_path_buf());

    while let Some(ref dir) = current {
        let canonical_dir = match path_within_base(dir, canonical_base) {
            Some(p) => p,
            None => break,
        };

        if canonical_dir == canonical_base {
            break;
        }

        match fs::remove_dir(&canonical_dir).await {
            Ok(()) => {
                current = dir.parent().map(|p| p.to_path_buf());
            }
            Err(_) => break,
        }
    }
}
