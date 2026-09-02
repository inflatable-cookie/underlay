//! Local reserved-metadata attachment for owned exclusive create.
//!
//! Extended attributes are written on the unpublished temp file descriptor
//! before `linkat` publishes the final name, so a reader of the destination
//! either sees the complete inode (bytes plus ownership facts) or nothing.
//!
//! Reads of reserved xattrs are bounded. An undersized `getxattr` buffer
//! returns `ERANGE` rather than the required length; this module never
//! treats a negative return as a size. Oversized or malformed reserved
//! attributes are omitted so `head`/`exists` keep their v0.9.6 contract.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io;
use std::os::unix::io::RawFd;
use std::path::Path;

use tracing::warn;

use crate::owned::{
    OwnedPublicationFacts, OWNED_META_MIME, OWNED_META_SHA256, OWNED_META_SIZE, OWNED_META_VERIFIER,
};

const XATTR_VERIFIER: &CStr = c"user.underlay.owned.v1.verifier";
const XATTR_SHA256: &CStr = c"user.underlay.owned.v1.sha256";
const XATTR_SIZE: &CStr = c"user.underlay.owned.v1.size";
const XATTR_MIME: &CStr = c"user.underlay.owned.v1.mime";

/// Maximum accepted reserved xattr value. Matches the MIME cap in
/// [`crate::owned::OwnedPublicationFacts::from_object_metadata`]. Verifier
/// and SHA-256 hex are 64 bytes; size is a decimal u64.
const MAX_RESERVED_XATTR_BYTES: usize = 128;

pub(super) fn set_owned_facts(fd: RawFd, facts: &OwnedPublicationFacts) -> io::Result<()> {
    for (meta_key, value) in facts.metadata_pairs() {
        let name = xattr_name(meta_key);
        sys::fsetxattr(fd, name, value.as_bytes())?;
    }
    Ok(())
}

/// Read reserved ownership xattrs. Never fails for hostile or oversized
/// reserved attributes: those keys are omitted so ordinary `head`/`exists`
/// callers do not observe a new IoError.
pub(super) fn read_owned_metadata(path: &Path) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    for meta_key in [
        OWNED_META_VERIFIER,
        OWNED_META_SHA256,
        OWNED_META_SIZE,
        OWNED_META_MIME,
    ] {
        match read_reserved_xattr(path, xattr_name(meta_key)) {
            ReservedXattr::Value(value) => {
                let Ok(text) = String::from_utf8(value) else {
                    continue;
                };
                metadata.insert(meta_key.to_string(), text);
            }
            ReservedXattr::Absent | ReservedXattr::Unproven => {}
            ReservedXattr::Unsupported => return HashMap::new(),
        }
    }
    metadata
}

enum ReservedXattr {
    Value(Vec<u8>),
    Absent,
    /// Oversized, ERANGE, or otherwise unreadable reserved attribute.
    Unproven,
    Unsupported,
}

fn xattr_name(meta_key: &str) -> &'static CStr {
    match meta_key {
        OWNED_META_VERIFIER => XATTR_VERIFIER,
        OWNED_META_SHA256 => XATTR_SHA256,
        OWNED_META_SIZE => XATTR_SIZE,
        OWNED_META_MIME => XATTR_MIME,
        _ => unreachable!("owned metadata key is one of the four reserved names"),
    }
}

fn read_reserved_xattr(path: &Path, name: &CStr) -> ReservedXattr {
    let Ok(path) = path_c_string(path) else {
        return ReservedXattr::Unproven;
    };
    let mut buf = vec![0u8; MAX_RESERVED_XATTR_BYTES];
    match sys::getxattr_nofollow(path.as_c_str(), name, buf.as_mut_ptr().cast(), buf.len()) {
        Ok(n) if n <= MAX_RESERVED_XATTR_BYTES => {
            buf.truncate(n);
            ReservedXattr::Value(buf)
        }
        Ok(_) => ReservedXattr::Unproven,
        Err(err) if is_missing_xattr(&err) => ReservedXattr::Absent,
        Err(err) if is_erange(&err) => ReservedXattr::Unproven,
        Err(err) if is_xattr_unsupported(&err) => ReservedXattr::Unsupported,
        Err(err) => {
            warn!(
                error = %err,
                "failed to read a reserved local ownership xattr; treating it as unproven"
            );
            ReservedXattr::Unproven
        }
    }
}

fn path_c_string(path: &Path) -> io::Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains a NUL byte"))
}

fn is_missing_xattr(err: &io::Error) -> bool {
    match err.raw_os_error() {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        Some(libc::ENOATTR) => true,
        #[cfg(any(target_os = "linux", target_os = "android"))]
        Some(libc::ENODATA) => true,
        _ => false,
    }
}

fn is_erange(err: &io::Error) -> bool {
    err.raw_os_error() == Some(libc::ERANGE)
}

fn is_xattr_unsupported(err: &io::Error) -> bool {
    if err.kind() == io::ErrorKind::Unsupported {
        return true;
    }
    err.raw_os_error()
        .is_some_and(|code| code == libc::ENOTSUP || code == libc::EOPNOTSUPP)
}

pub(super) fn log_xattr_failure(err: &io::Error) {
    warn!(
        error = %err,
        "local filesystem refused owned publication metadata; destination was not published"
    );
}

/// Platform xattr syscalls. Each arm returns; the unsupported-unix fallback
/// is a complete function so FreeBSD/illumos-style `cfg(unix)` targets
/// compile and fail closed.
mod sys {
    use super::*;

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub(super) fn fsetxattr(fd: RawFd, name: &CStr, value: &[u8]) -> io::Result<()> {
        // SAFETY: `fd` is a live file descriptor owned by the caller.
        // `name` is a static NUL-terminated CStr. `value` points at
        // `value.len()` readable bytes in this allocation for the duration
        // of the call. position is 0 (resource-fork offset unused for these
        // names) and flags are 0.
        let rc =
            unsafe { libc::fsetxattr(fd, name.as_ptr(), value.as_ptr().cast(), value.len(), 0, 0) };
        if rc == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub(super) fn fsetxattr(fd: RawFd, name: &CStr, value: &[u8]) -> io::Result<()> {
        // SAFETY: `fd` is a live file descriptor owned by the caller.
        // `name` is a static NUL-terminated CStr. `value` points at
        // `value.len()` readable bytes in this allocation for the duration
        // of the call. flags are 0 (create-or-replace).
        let rc =
            unsafe { libc::fsetxattr(fd, name.as_ptr(), value.as_ptr().cast(), value.len(), 0) };
        if rc == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "linux",
        target_os = "android"
    )))]
    pub(super) fn fsetxattr(fd: RawFd, name: &CStr, value: &[u8]) -> io::Result<()> {
        let _ = (fd, name, value);
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub(super) fn getxattr_nofollow(
        path: &CStr,
        name: &CStr,
        ptr: *mut libc::c_void,
        len: usize,
    ) -> io::Result<usize> {
        // SAFETY: `path` and `name` are NUL-terminated CStrs. `ptr`
        // addresses `len` writable bytes in the caller buffer for the
        // call. XATTR_NOFOLLOW prevents following a symlink destination.
        let n = unsafe {
            libc::getxattr(
                path.as_ptr(),
                name.as_ptr(),
                ptr,
                len,
                0,
                libc::XATTR_NOFOLLOW,
            )
        };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub(super) fn getxattr_nofollow(
        path: &CStr,
        name: &CStr,
        ptr: *mut libc::c_void,
        len: usize,
    ) -> io::Result<usize> {
        // SAFETY: `path` and `name` are NUL-terminated CStrs. `ptr`
        // addresses `len` writable bytes in the caller buffer for the
        // call. `lgetxattr` does not follow a symlink destination.
        let n = unsafe { libc::lgetxattr(path.as_ptr(), name.as_ptr(), ptr, len) };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "linux",
        target_os = "android"
    )))]
    pub(super) fn getxattr_nofollow(
        path: &CStr,
        name: &CStr,
        ptr: *mut libc::c_void,
        len: usize,
    ) -> io::Result<usize> {
        let _ = (path, name, ptr, len);
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}

/// Always-compiled copy of the unsupported-unix fallback bodies. Proves the
/// FreeBSD/illumos-style arm type-checks on every host; it is not a kernel
/// execution claim for those targets.
#[cfg(test)]
fn unsupported_unix_fsetxattr_fallback(fd: RawFd, name: &CStr, value: &[u8]) -> io::Result<()> {
    let _ = (fd, name, value);
    Err(io::Error::from(io::ErrorKind::Unsupported))
}

#[cfg(test)]
fn unsupported_unix_getxattr_fallback(
    path: &CStr,
    name: &CStr,
    ptr: *mut libc::c_void,
    len: usize,
) -> io::Result<usize> {
    let _ = (path, name, ptr, len);
    Err(io::Error::from(io::ErrorKind::Unsupported))
}

#[cfg(test)]
pub(super) fn plant_raw_reserved_xattr(
    path: &Path,
    meta_key: &str,
    value: &[u8],
) -> io::Result<()> {
    use std::os::unix::io::AsRawFd;
    let file = std::fs::OpenOptions::new().write(true).open(path)?;
    sys::fsetxattr(file.as_raw_fd(), xattr_name(meta_key), value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::owned::{OwnedDestinationAuthority, OwnershipToken};
    use crate::types::BlobObjectKey;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;

    fn test_authority(key: &str) -> OwnedDestinationAuthority {
        OwnedDestinationAuthority::new("local", "local-bucket", BlobObjectKey::parse(key).unwrap())
            .unwrap()
    }

    #[test]
    fn xattrs_are_visible_on_the_unpublished_inode_before_the_final_name() {
        let dir = std::env::temp_dir().join("underlay-blob-xattr-unpublished-probe");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let tmp = dir.join(".underlay-tmp.probe");
        let final_path = dir.join("final.bin");

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .unwrap();
        file.write_all(b"payload-bytes").unwrap();

        let token = OwnershipToken::from_bytes(vec![0x44; 32]).unwrap();
        let authority = test_authority("media/final.bin");
        let facts = OwnedPublicationFacts::from_token_and_bytes(
            &token,
            &authority,
            b"payload-bytes",
            "image/png",
        );
        set_owned_facts(file.as_raw_fd(), &facts).expect("fsetxattr must work on this filesystem");
        file.sync_all().unwrap();
        drop(file);

        assert!(!final_path.exists(), "final name must still be absent");
        let unpublished = read_owned_metadata(&tmp);
        let parsed = OwnedPublicationFacts::from_object_metadata(&unpublished).unwrap();
        assert!(parsed.matches_token(&token, &authority));

        std::fs::hard_link(&tmp, &final_path).unwrap();
        let published = read_owned_metadata(&final_path);
        assert_eq!(unpublished, published);
        assert_eq!(std::fs::read(&final_path).unwrap(), b"payload-bytes");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn oversized_reserved_xattr_is_omitted_not_an_io_error() {
        let dir = std::env::temp_dir().join("underlay-blob-xattr-oversized-probe");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("legacy.bin");
        std::fs::write(&path, b"legacy-bytes").unwrap();

        let oversized = vec![0x41u8; MAX_RESERVED_XATTR_BYTES + 32];
        plant_raw_reserved_xattr(&path, OWNED_META_VERIFIER, &oversized)
            .expect("filesystem must accept an oversized xattr for this probe");

        let metadata = read_owned_metadata(&path);
        assert!(
            !metadata.contains_key(OWNED_META_VERIFIER),
            "oversized reserved xattr must be treated as unproven, not returned"
        );
        assert!(OwnedPublicationFacts::from_object_metadata(&metadata).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unsupported_unix_cfg_arm_typechecks_and_fails_closed() {
        let name = c"user.underlay.owned.v1.verifier";
        let err = unsupported_unix_fsetxattr_fallback(-1, name, b"").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Unsupported);
        let err =
            unsupported_unix_getxattr_fallback(name, name, std::ptr::null_mut(), 0).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Unsupported);
    }
}
