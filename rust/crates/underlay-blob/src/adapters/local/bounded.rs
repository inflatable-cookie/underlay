//! Bounded, containment-safe local I/O backing verified promotion.
//!
//! Every containment guarantee here comes from the syscall that performs
//! the operation, never from a check performed before it. On Unix each path
//! component is opened relative to the previous directory descriptor with
//! `openat(..., O_NOFOLLOW)`: a concurrent replacement of a not-yet-opened
//! component with a symlink cannot redirect traversal outside the base,
//! because the same syscall that would follow it is the one that refuses
//! it. There is no lexical path resolved ahead of time to race against.
//!
//! On platforms without that primitive, both operations fail closed with
//! [`BlobError::Unsupported`] rather than falling back to a weaker
//! check-then-act resolution while still claiming no-follow behavior.

use std::path::Path;

use crate::error::BlobResult;

/// Read at most `max_bytes + 1` bytes from `key` under `canonical_base`.
///
/// Refuses symlinks and non-regular files (directories, FIFOs, devices)
/// without blocking: the descriptor is opened `O_NOFOLLOW | O_NONBLOCK`
/// (open on a FIFO with `O_NONBLOCK` never blocks, even without a writer),
/// then `fstat`-checked before any `read` is attempted. `key` is used for
/// error messages only; the resolved path never leaks into a returned
/// error.
pub(super) async fn read_bounded(
    canonical_base: &Path,
    key: &str,
    max_bytes: u64,
) -> BlobResult<Vec<u8>> {
    let canonical_base = canonical_base.to_path_buf();
    let key = key.to_string();
    run_blocking(move || imp::read_bounded_sync(&canonical_base, &key, max_bytes)).await
}

/// Create `key` under `canonical_base` if and only if it does not already
/// exist, then write `data` to it.
///
/// Uses `O_CREAT | O_EXCL` (plus `O_NOFOLLOW` on Unix as defense in depth):
/// an existing file, symlink (dangling or not), or directory at the
/// destination all make the create call fail atomically with no write
/// attempted. Missing parent directories are created the same
/// descriptor-relative, no-follow way. Never truncates or follows an
/// occupied destination. `key` is used for error messages only.
pub(super) async fn create_only(canonical_base: &Path, key: &str, data: &[u8]) -> BlobResult<()> {
    let canonical_base = canonical_base.to_path_buf();
    let key = key.to_string();
    let data = data.to_vec();
    run_blocking(move || imp::create_only_sync(&canonical_base, &key, &data)).await
}

async fn run_blocking<F, T>(f: F) -> BlobResult<T>
where
    F: FnOnce() -> BlobResult<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(result) => result,
        Err(join_error) => Err(crate::error::BlobError::IoError(format!(
            "bounded local I/O task did not complete: {join_error}"
        ))),
    }
}

#[cfg(unix)]
mod imp {
    use std::ffi::{CString, OsStr};
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
    use std::path::{Component, Path};

    use crate::error::{BlobError, BlobResult};

    fn to_cstring(component: &OsStr) -> BlobResult<CString> {
        CString::new(component.as_bytes())
            .map_err(|_| BlobError::InvalidKey("key contains a NUL byte".to_string()))
    }

    fn last_os_error_as_blob_error(err: &io::Error, key: &str, on_missing: BlobError) -> BlobError {
        match err.raw_os_error() {
            Some(libc::ENOENT) => on_missing,
            Some(libc::ELOOP) | Some(libc::ENOTDIR) => {
                BlobError::InvalidKey("key path escapes the base directory".to_string())
            }
            _ => {
                let _ = key;
                BlobError::IoError(err.to_string())
            }
        }
    }

    /// Open (or, if `create_missing`, create-then-open) `name` as a
    /// directory relative to `dir_fd`. The `openat(O_NOFOLLOW |
    /// O_DIRECTORY)` call is the entire check: it is refused atomically if
    /// `name` is a symlink or a non-directory, however recently it was
    /// planted.
    fn open_dir_component(dir_fd: RawFd, name: &CString, create_missing: bool) -> io::Result<File> {
        let flags = libc::O_DIRECTORY | libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC;
        let fd = unsafe { libc::openat(dir_fd, name.as_ptr(), flags, 0) };
        if fd >= 0 {
            return Ok(unsafe { File::from_raw_fd(fd) });
        }

        let err = io::Error::last_os_error();
        if !create_missing || err.raw_os_error() != Some(libc::ENOENT) {
            return Err(err);
        }

        // Create then open. Each call is independently atomic with respect
        // to symlinks/existence: a concurrent creator of the same real
        // directory only costs us the `mkdirat` race (`EEXIST`, ignored
        // here), and a concurrent symlink plant is still refused by the
        // `O_NOFOLLOW` open that follows.
        if unsafe { libc::mkdirat(dir_fd, name.as_ptr(), 0o755) } != 0 {
            let mkdir_err = io::Error::last_os_error();
            if mkdir_err.raw_os_error() != Some(libc::EEXIST) {
                return Err(mkdir_err);
            }
        }

        let fd = unsafe { libc::openat(dir_fd, name.as_ptr(), flags, 0) };
        if fd >= 0 {
            Ok(unsafe { File::from_raw_fd(fd) })
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Split `key` into validated normal components, then walk from
    /// `canonical_base` to the parent directory of the final component,
    /// opening/creating each intermediate directory relative to the
    /// previous descriptor. No lexical path is ever resolved or trusted as
    /// a whole; each step's containment guarantee is that step's own
    /// syscall.
    fn descend_to_parent(
        canonical_base: &Path,
        key: &str,
        create_missing: bool,
    ) -> BlobResult<(File, CString)> {
        let components: Vec<&OsStr> = Path::new(key)
            .components()
            .map(|component| match component {
                Component::Normal(part) => Ok(part),
                _ => Err(BlobError::InvalidKey(
                    "key path escapes the base directory".to_string(),
                )),
            })
            .collect::<Result<_, _>>()?;

        let (last, parents) = components
            .split_last()
            .ok_or_else(|| BlobError::InvalidKey("key must not be empty".to_string()))?;

        let base_cpath = to_cstring(canonical_base.as_os_str())?;
        let base_fd = unsafe {
            libc::open(
                base_cpath.as_ptr(),
                libc::O_DIRECTORY | libc::O_RDONLY | libc::O_CLOEXEC,
            )
        };
        if base_fd < 0 {
            return Err(BlobError::IoError(io::Error::last_os_error().to_string()));
        }
        let mut current = unsafe { File::from_raw_fd(base_fd) };

        for component in parents {
            let name = to_cstring(component)?;
            current =
                open_dir_component(current.as_raw_fd(), &name, create_missing).map_err(|err| {
                    last_os_error_as_blob_error(&err, key, BlobError::NotFound(key.to_string()))
                })?;
        }

        let last_name = to_cstring(last)?;
        Ok((current, last_name))
    }

    pub(super) fn read_bounded_sync(
        canonical_base: &Path,
        key: &str,
        max_bytes: u64,
    ) -> BlobResult<Vec<u8>> {
        let (parent, name) = descend_to_parent(canonical_base, key, false)?;

        let flags = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
        let fd = unsafe { libc::openat(parent.as_raw_fd(), name.as_ptr(), flags, 0) };
        let mut file = if fd >= 0 {
            unsafe { File::from_raw_fd(fd) }
        } else {
            let err = io::Error::last_os_error();
            return Err(match err.raw_os_error() {
                Some(libc::ENOENT) => BlobError::NotFound(key.to_string()),
                Some(libc::ELOOP) => {
                    BlobError::Unsupported("source is not a regular file".to_string())
                }
                _ => BlobError::IoError(err.to_string()),
            });
        };

        // The descriptor is now pinned to whatever `openat` actually
        // resolved at that instant; nothing that happens to the directory
        // entry afterward can change what `fstat`/`read` observe below.
        let metadata = file
            .metadata()
            .map_err(|e| BlobError::IoError(e.to_string()))?;
        if !metadata.is_file() {
            return Err(BlobError::Unsupported(
                "source is not a regular file".to_string(),
            ));
        }

        let cap = max_bytes.saturating_add(1);
        let mut buf = Vec::with_capacity(cap.min(8 * 1024 * 1024) as usize);
        (&mut file)
            .take(cap)
            .read_to_end(&mut buf)
            .map_err(|e| BlobError::IoError(e.to_string()))?;

        if buf.len() as u64 > max_bytes {
            return Err(BlobError::TooLarge(buf.len() as u64, max_bytes));
        }

        Ok(buf)
    }

    pub(super) fn create_only_sync(
        canonical_base: &Path,
        key: &str,
        data: &[u8],
    ) -> BlobResult<()> {
        let (parent, name) = descend_to_parent(canonical_base, key, true)?;

        let flags =
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC;
        let fd = unsafe { libc::openat(parent.as_raw_fd(), name.as_ptr(), flags, 0o644) };
        let mut file = if fd >= 0 {
            unsafe { File::from_raw_fd(fd) }
        } else {
            let err = io::Error::last_os_error();
            return Err(match err.raw_os_error() {
                Some(libc::EEXIST) | Some(libc::ELOOP) => {
                    BlobError::DestinationExists(key.to_string())
                }
                _ => BlobError::IoError(err.to_string()),
            });
        };

        file.write_all(data)
            .map_err(|e| BlobError::IoError(format!("failed to write file: {e}")))?;
        file.sync_all()
            .map_err(|e| BlobError::IoError(format!("failed to flush file: {e}")))?;

        Ok(())
    }
}

#[cfg(not(unix))]
mod imp {
    use std::path::Path;

    use crate::error::{BlobError, BlobResult};

    pub(super) fn read_bounded_sync(
        _canonical_base: &Path,
        _key: &str,
        _max_bytes: u64,
    ) -> BlobResult<Vec<u8>> {
        Err(BlobError::Unsupported(
            "local bounded capture requires no-follow/non-blocking open support unavailable on this platform"
                .to_string(),
        ))
    }

    pub(super) fn create_only_sync(
        _canonical_base: &Path,
        _key: &str,
        _data: &[u8],
    ) -> BlobResult<()> {
        Err(BlobError::Unsupported(
            "local exclusive create requires no-follow open support unavailable on this platform"
                .to_string(),
        ))
    }
}
