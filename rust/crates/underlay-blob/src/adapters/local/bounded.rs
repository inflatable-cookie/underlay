//! Bounded, containment-safe local I/O backing verified promotion.
//!
//! Both helpers operate on a path already resolved and containment-checked
//! by [`super::path::joined_path_within_base`]. They add the remaining
//! safety properties `promote_verified` depends on: a source read that never
//! follows a symlink or blocks on a non-regular file, and a destination
//! write that never overwrites, truncates, or follows an existing path.

use std::path::Path;

use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::error::{BlobError, BlobResult};

#[cfg(unix)]
fn exclusive_create_options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options
        .write(true)
        .create_new(true)
        .custom_flags(libc::O_NOFOLLOW);
    options
}

#[cfg(not(unix))]
fn exclusive_create_options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    options
}

#[cfg(unix)]
fn read_only_no_follow_options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options.read(true).custom_flags(libc::O_NOFOLLOW);
    options
}

#[cfg(not(unix))]
fn read_only_no_follow_options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options.read(true);
    options
}

/// Read at most `max_bytes + 1` bytes from `path`.
///
/// Refuses symlinks and non-regular files (directories, FIFOs, devices)
/// without blocking: the file type is checked with `lstat` before any
/// `open`/`read` is attempted, so a named pipe with no writer never hangs
/// the calling task. `key` is used for error messages only; `path` never
/// leaks into a returned error.
pub(super) async fn read_bounded(path: &Path, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
    let lstat = fs::symlink_metadata(path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            BlobError::NotFound(key.to_string())
        } else {
            BlobError::IoError(e.to_string())
        }
    })?;

    if !lstat.is_file() {
        return Err(BlobError::Unsupported(
            "source is not a regular file".to_string(),
        ));
    }

    let mut file = read_only_no_follow_options()
        .open(path)
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })?;

    // Re-check on the open descriptor: closes the lstat-to-open race window
    // for a swap into a non-regular file without ever calling read() on it.
    let fstat = file
        .metadata()
        .await
        .map_err(|e| BlobError::IoError(e.to_string()))?;
    if !fstat.is_file() {
        return Err(BlobError::Unsupported(
            "source is not a regular file".to_string(),
        ));
    }

    let cap = max_bytes.saturating_add(1);
    let mut buf = Vec::with_capacity(cap.min(8 * 1024 * 1024) as usize);
    (&mut file)
        .take(cap)
        .read_to_end(&mut buf)
        .await
        .map_err(|e| BlobError::IoError(e.to_string()))?;

    if buf.len() as u64 > max_bytes {
        return Err(BlobError::TooLarge(buf.len() as u64, max_bytes));
    }

    Ok(buf)
}

/// Create `path` if and only if it does not already exist, then write
/// `data` to it.
///
/// Uses `O_CREAT | O_EXCL` (plus `O_NOFOLLOW` on Unix as defense in depth):
/// an existing file, symlink (dangling or not), or directory at `path` all
/// make the create call fail atomically with no write attempted. Never
/// truncates or follows an occupied destination. `key` is used for error
/// messages only.
pub(super) async fn create_only(path: &Path, key: &str, data: &[u8]) -> BlobResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| BlobError::IoError(format!("failed to create directories: {e}")))?;
    }

    let mut file = match exclusive_create_options().open(path).await {
        Ok(file) => file,
        // `O_CREAT | O_EXCL` refuses on any existing dirent at `path` —
        // file, directory, or symlink (dangling or not) — without
        // following it. `O_NOFOLLOW` (set above on Unix) is defense in
        // depth for the same case.
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(BlobError::DestinationExists(key.to_string()));
        }
        Err(e) => return Err(BlobError::IoError(e.to_string())),
    };

    file.write_all(data)
        .await
        .map_err(|e| BlobError::IoError(format!("failed to write file: {e}")))?;
    file.flush()
        .await
        .map_err(|e| BlobError::IoError(format!("failed to flush file: {e}")))?;

    Ok(())
}
