//! Bounded, containment-safe local I/O backing verified promotion.
//!
//! Every containment guarantee here comes from the syscall that performs
//! the operation, never from a check performed before it, starting from the
//! base directory itself: [`LocalAdapter`](super::adapter::LocalAdapter)
//! pins one owned descriptor to the base directory at construction, and
//! every call here descends from a duplicate of that descriptor rather than
//! re-opening the base by its (renameable, replaceable) pathname. From
//! there, each path component is opened relative to the previous directory
//! descriptor with `openat(..., O_NOFOLLOW)`: a concurrent replacement of a
//! not-yet-opened component with a symlink cannot redirect traversal
//! outside the base, because the same syscall that would follow it is the
//! one that refuses it. There is no lexical path resolved ahead of time to
//! race against, at any level from the base down.
//!
//! Publication is also never visible partially: `create_only` writes and
//! `fsync`s an owned, unguessable same-directory temporary file first, then
//! publishes it under the final name with `linkat` (an atomic,
//! create-if-absent namespace operation, exactly like the old direct
//! `O_CREAT | O_EXCL` open but performed on an inode that is already
//! complete and durable). A concurrent reader can only ever see "not found"
//! or "fully published"; a write failure, a destination collision, or a
//! process crash before `linkat` leaves only the caller-owned temporary
//! name behind, never a poisoned final name, so a retry after failure is
//! not permanently blocked.
//!
//! On platforms without these primitives, both operations fail closed with
//! [`BlobError::Unsupported`] rather than falling back to a weaker
//! check-then-act resolution while still claiming no-follow behavior.

use crate::error::BlobResult;

/// Read at most `max_bytes + 1` bytes from `key`, resolved relative to a
/// duplicate of the adapter's pinned base-directory descriptor.
///
/// Refuses symlinks and non-regular files (directories, FIFOs, devices)
/// without blocking: the descriptor is opened `O_NOFOLLOW | O_NONBLOCK`
/// (open on a FIFO with `O_NONBLOCK` never blocks, even without a writer),
/// then `fstat`-checked before any `read` is attempted. `key` is used for
/// error messages only; the resolved path never leaks into a returned
/// error.
#[cfg(unix)]
pub(super) async fn read_bounded(
    base_dir: &std::fs::File,
    key: &str,
    max_bytes: u64,
) -> BlobResult<Vec<u8>> {
    let base_dir = base_dir
        .try_clone()
        .map_err(|e| crate::error::BlobError::IoError(e.to_string()))?;
    let key = key.to_string();
    run_blocking(move || imp::read_bounded_sync(base_dir, &key, max_bytes)).await
}

#[cfg(not(unix))]
pub(super) async fn read_bounded(key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
    let _ = (key, max_bytes);
    Err(crate::error::BlobError::Unsupported(
        "local bounded capture requires no-follow/non-blocking open support unavailable on this platform"
            .to_string(),
    ))
}

/// Create `key` if and only if it does not already exist, then durably
/// publish `data` to it. `key` is resolved relative to a duplicate of the
/// adapter's pinned base-directory descriptor.
///
/// The destination is never visible with partial content: `data` is
/// written and `fsync`ed to an owned, unguessable same-directory temporary
/// file, then published under the final name with an atomic, exclusive
/// `linkat`. An existing file, symlink (dangling or not), or directory at
/// the destination makes the publish step fail atomically with the
/// incumbent untouched. The temporary file is removed on every path (a
/// successful publish leaves only the final name; a collision or write
/// failure removes just the caller's own temp, nothing else in the
/// directory). Missing parent directories are created the same
/// descriptor-relative, no-follow way. `key` is used for error messages
/// only.
#[cfg(unix)]
pub(super) async fn create_only(
    base_dir: &std::fs::File,
    key: &str,
    data: &[u8],
) -> BlobResult<()> {
    let base_dir = base_dir
        .try_clone()
        .map_err(|e| crate::error::BlobError::IoError(e.to_string()))?;
    let key = key.to_string();
    let data = data.to_vec();
    run_blocking(move || imp::create_only_sync(base_dir, &key, &data)).await
}

#[cfg(not(unix))]
pub(super) async fn create_only(key: &str, data: &[u8]) -> BlobResult<()> {
    let _ = (key, data);
    Err(crate::error::BlobError::Unsupported(
        "local exclusive create requires no-follow open support unavailable on this platform"
            .to_string(),
    ))
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

/// Open one owned descriptor to `canonical_base`, pinned once at adapter
/// construction. All later traversal descends from a duplicate of this
/// descriptor rather than re-opening the base by pathname, so a rename of
/// the base directory (with its old pathname replaced by a symlink)
/// afterward cannot redirect any operation outside it.
#[cfg(unix)]
pub(super) fn open_pinned_base_dir(
    canonical_base: &std::path::Path,
) -> crate::error::BlobResult<std::fs::File> {
    use std::os::unix::ffi::OsStrExt;

    let c_path = std::ffi::CString::new(canonical_base.as_os_str().as_bytes()).map_err(|_| {
        crate::error::BlobError::ConfigError("base path contains a NUL byte".to_string())
    })?;
    let fd = unsafe {
        libc::open(
            c_path.as_ptr(),
            libc::O_DIRECTORY | libc::O_RDONLY | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(crate::error::BlobError::ConfigError(format!(
            "Failed to open base directory: {}",
            std::io::Error::last_os_error()
        )));
    }
    Ok(unsafe { <std::fs::File as std::os::unix::io::FromRawFd>::from_raw_fd(fd) })
}

#[cfg(unix)]
mod imp {
    use std::ffi::{CString, OsStr};
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
    use std::path::Component;

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
    /// `base_dir` (an owned descriptor already pinned to the base
    /// directory, never re-opened by pathname here) to the parent
    /// directory of the final component, opening/creating each
    /// intermediate directory relative to the previous descriptor. No
    /// lexical path is ever resolved or trusted as a whole; each step's
    /// containment guarantee is that step's own syscall.
    fn descend_to_parent(
        base_dir: File,
        key: &str,
        create_missing: bool,
    ) -> BlobResult<(File, CString)> {
        let components: Vec<&OsStr> = std::path::Path::new(key)
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

        let mut current = base_dir;

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
        base_dir: File,
        key: &str,
        max_bytes: u64,
    ) -> BlobResult<Vec<u8>> {
        let (parent, name) = descend_to_parent(base_dir, key, false)?;

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

    /// Best-effort removal of the caller's own temporary name. Never
    /// touches anything else in the directory; a failure here (the temp
    /// was already gone, or some other transient error) is not itself
    /// treated as fatal since the caller has an unrelated error or success
    /// to report already.
    fn cleanup_temp(parent_fd: RawFd, temp_name: &CString) {
        unsafe {
            libc::unlinkat(parent_fd, temp_name.as_ptr(), 0);
        }
    }

    fn temp_name() -> BlobResult<CString> {
        let candidate = format!(
            ".underlay-tmp.{}.{:016x}",
            std::process::id(),
            rand::random::<u64>()
        );
        to_cstring(OsStr::new(&candidate))
    }

    pub(super) fn create_only_sync(base_dir: File, key: &str, data: &[u8]) -> BlobResult<()> {
        let (parent, final_name) = descend_to_parent(base_dir, key, true)?;
        let parent_fd = parent.as_raw_fd();

        // Write and durably sync an owned, unguessable same-directory temp
        // name first. Nothing is visible under `final_name` yet, so a
        // concurrent reader of the destination can only ever see "not
        // found" or "fully published", never a partial write.
        let temp_name = temp_name()?;
        let temp_flags =
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC;
        let temp_fd = unsafe { libc::openat(parent_fd, temp_name.as_ptr(), temp_flags, 0o600) };
        let mut temp_file = if temp_fd >= 0 {
            unsafe { File::from_raw_fd(temp_fd) }
        } else {
            return Err(BlobError::IoError(io::Error::last_os_error().to_string()));
        };

        if let Err(e) = temp_file.write_all(data) {
            cleanup_temp(parent_fd, &temp_name);
            return Err(BlobError::IoError(format!("failed to write file: {e}")));
        }
        if let Err(e) = temp_file.sync_all() {
            cleanup_temp(parent_fd, &temp_name);
            return Err(BlobError::IoError(format!("failed to flush file: {e}")));
        }
        drop(temp_file);

        // Publish atomically: `linkat` adds a second directory entry for
        // the now-complete, durable inode, and fails with `EEXIST` if
        // `final_name` is already occupied — the same exclusive-create
        // guarantee `O_CREAT | O_EXCL` gave directly before, but now
        // nothing under `final_name` is ever partially written, and a
        // collision or failure here leaves only the caller-owned temp
        // name to clean up, never a poisoned final name.
        let link_ret = unsafe {
            libc::linkat(
                parent_fd,
                temp_name.as_ptr(),
                parent_fd,
                final_name.as_ptr(),
                0,
            )
        };

        if link_ret != 0 {
            let err = io::Error::last_os_error();
            cleanup_temp(parent_fd, &temp_name);
            return Err(match err.raw_os_error() {
                Some(libc::EEXIST) | Some(libc::ELOOP) => {
                    BlobError::DestinationExists(key.to_string())
                }
                _ => BlobError::IoError(err.to_string()),
            });
        }

        // `final_name` now owns a link to the inode; drop the temp name
        // (a second link to the same inode, not the caller's bytes).
        cleanup_temp(parent_fd, &temp_name);

        Ok(())
    }
}
