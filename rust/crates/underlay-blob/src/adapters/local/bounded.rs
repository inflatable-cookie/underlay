//! Bounded, containment-safe local I/O backing verified promotion.
//!
//! Every containment guarantee here comes from the syscall that performs
//! the operation, never from a check performed before it — starting from
//! the base directory itself. [`LocalAdapter`](super::adapter::LocalAdapter)
//! pins one owned descriptor to the base directory at construction by
//! walking its canonical absolute path one component at a time with
//! `openat(O_DIRECTORY | O_NOFOLLOW)` from an owned root descriptor, never
//! by handing a lexical path string to a single `open()` call. Every later
//! bounded/exclusive operation descends from a duplicate of that pinned
//! descriptor rather than re-opening the base by pathname, and from there
//! each key component is opened relative to the previous directory
//! descriptor the same `openat(..., O_NOFOLLOW)` way: a concurrent
//! replacement of a not-yet-opened component with a symlink cannot
//! redirect traversal outside the base, because the same syscall that
//! would follow it is the one that refuses it. There is no lexical path
//! resolved ahead of time to race against, at any level from construction
//! down.
//!
//! Publication is also never visible partially: `create_only` writes and
//! `fsync`s an owned, unguessable same-directory temporary file first, then
//! publishes it under the final name with `linkat` (an atomic,
//! create-if-absent namespace operation, exactly like the old direct
//! `O_CREAT | O_EXCL` open but performed on an inode that is already
//! complete and durable), then `fsync`s the parent directory so the new
//! name itself is durable, not only the bytes behind it. Owned exclusive
//! create attaches reserved metadata to that unpublished temp inode before
//! the `fsync` and `linkat`, so a reader of the final name either sees the
//! complete object plus ownership facts or neither. A concurrent
//! reader can only ever see "not found" or "fully published"; a write
//! failure or a destination collision before `linkat` leaves only the
//! caller-owned temporary name to remove, never a poisoned final name, so a
//! retry after failure is not permanently blocked. Once `linkat` reports
//! success the publish is never re-reported as a failure: the temp-file
//! cleanup and the parent-directory `fsync` that follow it are both
//! best-effort and their outcome is logged, not returned, so a caller can
//! never observe an error for a destination that may already be
//! committed — see [`imp::create_only_sync`] for the exact boundary.
//!
//! On platforms without these primitives, both operations fail closed with
//! [`BlobError::Unsupported`] rather than falling back to a weaker
//! check-then-act resolution while still claiming no-follow behavior.

use crate::error::BlobResult;
use crate::owned::OwnedPublicationFacts;

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

/// Create `key` if and only if it does not already exist, then publish
/// `data` to it. `key` is resolved relative to a duplicate of the
/// adapter's pinned base-directory descriptor.
///
/// The destination is never visible with partial content: `data` is
/// written and `fsync`ed to an owned, unguessable same-directory temporary
/// file, then published under the final name with an atomic, exclusive
/// `linkat`, then the parent directory is `fsync`ed so the new name itself
/// is durable. An existing file, symlink (dangling or not), or directory
/// at the destination makes the publish step fail atomically with the
/// incumbent untouched. Once `linkat` reports success this call cannot
/// fail: the temp-file removal and the parent `fsync` that follow are
/// best-effort (their outcome is logged, never returned), so a caller
/// never sees an error for a destination that may already be committed. A
/// leftover temp file from a best-effort cleanup failure never blocks a
/// later `create_only` call, for this key or any other — collision
/// detection is keyed on the final name only. Missing parent directories
/// are created the same descriptor-relative, no-follow way. `key` is used
/// for error messages only.
///
/// This adapter is a narrow local-filesystem dev/utility seam, not a
/// production durability guarantee: the parent-directory `fsync` gives the
/// new name the same crash-durability POSIX local filesystems normally
/// provide for an `fsync`ed directory, but that is a best-effort
/// improvement over the pre-existing behavior, not a cross-filesystem
/// (e.g. network/overlay) guarantee.
#[cfg(unix)]
pub(super) async fn create_only(
    base_dir: &std::fs::File,
    key: &str,
    data: &[u8],
    owned: Option<OwnedPublicationFacts>,
) -> BlobResult<()> {
    let base_dir = base_dir
        .try_clone()
        .map_err(|e| crate::error::BlobError::IoError(e.to_string()))?;
    let key = key.to_string();
    let data = data.to_vec();
    run_blocking(move || imp::create_only_sync(base_dir, &key, &data, owned.as_ref())).await
}

#[cfg(not(unix))]
pub(super) async fn create_only(
    key: &str,
    data: &[u8],
    owned: Option<OwnedPublicationFacts>,
) -> BlobResult<()> {
    let _ = (key, data, owned);
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
/// construction, by walking its canonical absolute path one component at a
/// time from an owned root descriptor with `openat(O_DIRECTORY |
/// O_NOFOLLOW)`. This is deliberately *not* a single `open()` call on the
/// path string: `canonicalize()` and this call are necessarily two
/// separate steps, and a plain `open()` here would let any component
/// replaced with a symlink in between be silently followed. Walking with
/// `O_NOFOLLOW` at every step means that window closes the same way the
/// per-key traversal below does — the containment guarantee is each step's
/// own syscall, not a check performed before it.
#[cfg(unix)]
pub(super) fn open_pinned_base_dir(
    canonical_base: &std::path::Path,
) -> crate::error::BlobResult<std::fs::File> {
    imp::open_pinned_base_dir_sync(canonical_base)
}

#[cfg(unix)]
mod imp {
    use std::ffi::{CString, OsStr};
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
    use std::path::Component;

    use tracing::warn;

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

    /// Open an owned descriptor to the filesystem root (`/`). The
    /// component-wise traversal below starts here rather than trusting any
    /// lexical prefix of the configured base path.
    fn open_root() -> BlobResult<File> {
        let root = CString::new("/").expect("static path literal has no NUL byte");
        let fd = unsafe {
            libc::open(
                root.as_ptr(),
                libc::O_DIRECTORY | libc::O_RDONLY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(BlobError::ConfigError(format!(
                "Failed to open filesystem root: {}",
                io::Error::last_os_error()
            )));
        }
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    /// Walk `canonical_base` (an absolute path) from an owned root
    /// descriptor, one component at a time, with `openat(O_DIRECTORY |
    /// O_NOFOLLOW)`. `canonicalize()` (which necessarily inspects the
    /// filesystem as a separate, prior step) can never make this call
    /// trust a symlink planted afterward: each component here is either a
    /// real directory this call opens itself, or the call fails.
    pub(super) fn open_pinned_base_dir_sync(canonical_base: &std::path::Path) -> BlobResult<File> {
        if !canonical_base.is_absolute() {
            return Err(BlobError::ConfigError(
                "base path must canonicalize to an absolute path".to_string(),
            ));
        }

        let mut current = open_root()?;

        for component in canonical_base.components() {
            match component {
                Component::RootDir | Component::Prefix(_) => continue,
                Component::CurDir | Component::ParentDir => {
                    return Err(BlobError::ConfigError(
                        "canonicalized base path must not contain '.' or '..' components"
                            .to_string(),
                    ));
                }
                Component::Normal(part) => {
                    let name = to_cstring(part).map_err(|_| {
                        BlobError::ConfigError("base path contains a NUL byte".to_string())
                    })?;
                    current =
                        open_dir_component(current.as_raw_fd(), &name, false).map_err(|err| {
                            BlobError::ConfigError(format!(
                                "Failed to open base directory component: {err}"
                            ))
                        })?;
                }
            }
        }

        Ok(current)
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

    /// Best-effort removal of the caller's own temporary name.
    ///
    /// This never affects whether `create_only_sync` reports success or
    /// failure: by the time it is called, that outcome is already decided
    /// by whether `linkat` published the final name. It touches only the
    /// exact temp name this call generated for itself — never a scan or
    /// glob of the directory — so it can never remove anything another
    /// call or a prior crashed attempt left behind. A failure here (most
    /// commonly `ENOENT`, which is not logged) leaves an orphaned temp
    /// file; that file plays no further role, since collision detection
    /// and publication are both keyed on the final name only, never on a
    /// temp name.
    fn cleanup_temp(parent_fd: RawFd, temp_name: &CString) {
        if unsafe { libc::unlinkat(parent_fd, temp_name.as_ptr(), 0) } != 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() != Some(libc::ENOENT) {
                warn!(
                    error = %err,
                    "failed to remove local blob temp file after publish; it is orphaned \
                     but does not affect destination correctness, collision detection, or \
                     future retries"
                );
            }
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

    pub(super) fn create_only_sync(
        base_dir: File,
        key: &str,
        data: &[u8],
        owned: Option<&crate::owned::OwnedPublicationFacts>,
    ) -> BlobResult<()> {
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
        if let Some(facts) = owned {
            if let Err(err) = super::super::xattr::set_owned_facts(temp_file.as_raw_fd(), facts) {
                super::super::xattr::log_xattr_failure(&err);
                cleanup_temp(parent_fd, &temp_name);
                return Err(BlobError::Unsupported(
                    "local filesystem cannot attach owned publication metadata".to_string(),
                ));
            }
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
        // name to clean up, never a poisoned final name. `linkat`'s
        // success/failure return is authoritative on the local POSIX
        // filesystems this dev/utility adapter targets: a nonzero return
        // means no new link was created.
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

        // The publish is committed from here on: `final_name` already
        // owns a link to the complete inode, so this call must not report
        // failure past this point — a caller that saw an error and
        // retried against a destination that actually exists would hit an
        // unexplained `DestinationExists` on a "failed" write, exactly
        // the poisoned-retry hazard this design exists to avoid.
        //
        // `fsync` the parent directory so the new directory entry itself
        // — not just the bytes behind it — survives a crash. This is a
        // best-effort durability improvement, not a new success
        // condition: a failure here is logged, never returned.
        if unsafe { libc::fsync(parent_fd) } != 0 {
            warn!(
                error = %io::Error::last_os_error(),
                "failed to fsync parent directory after publishing a local blob destination; \
                 the publish itself succeeded and is visible, but the new directory entry's \
                 crash-durability is not guaranteed until a later successful fsync of this \
                 directory"
            );
        }

        // `final_name` now owns a link to the inode; drop the temp name
        // (a second link to the same inode, not the caller's bytes).
        // Best-effort, as documented on `cleanup_temp`.
        cleanup_temp(parent_fd, &temp_name);

        Ok(())
    }
}
