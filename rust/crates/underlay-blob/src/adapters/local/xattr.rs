//! Local reserved-metadata attachment for owned exclusive create.
//!
//! Extended attributes are written on the unpublished temp file descriptor
//! before `linkat` publishes the final name, so a reader of the destination
//! either sees the complete inode (bytes plus ownership facts) or nothing.

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

pub(super) fn set_owned_facts(fd: RawFd, facts: &OwnedPublicationFacts) -> io::Result<()> {
    for (meta_key, value) in facts.metadata_pairs() {
        let name = xattr_name(meta_key);
        fsetxattr(fd, name, value.as_bytes())?;
    }
    Ok(())
}

pub(super) fn read_owned_metadata(path: &Path) -> io::Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    for meta_key in [
        OWNED_META_VERIFIER,
        OWNED_META_SHA256,
        OWNED_META_SIZE,
        OWNED_META_MIME,
    ] {
        match getxattr_nofollow(path, xattr_name(meta_key)) {
            Ok(value) => {
                let Ok(text) = String::from_utf8(value) else {
                    continue;
                };
                metadata.insert(meta_key.to_string(), text);
            }
            Err(err) if is_missing_xattr(&err) => {}
            Err(err) if is_xattr_unsupported(&err) => return Ok(HashMap::new()),
            Err(err) => return Err(err),
        }
    }
    Ok(metadata)
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

fn fsetxattr(fd: RawFd, name: &CStr, value: &[u8]) -> io::Result<()> {
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        let _ = (fd, name, value);
        return Err(io::Error::from(io::ErrorKind::Unsupported));
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let rc = {
        // SAFETY: `fd` is a live file descriptor owned by the caller.
        // `name` is a static NUL-terminated CStr. `value` points at
        // `value.len()` readable bytes in this allocation for the duration
        // of the call. position is 0 (resource-fork offset unused for these
        // names) and flags are 0.
        unsafe { libc::fsetxattr(fd, name.as_ptr(), value.as_ptr().cast(), value.len(), 0, 0) }
    };
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let rc = {
        // SAFETY: `fd` is a live file descriptor owned by the caller.
        // `name` is a static NUL-terminated CStr. `value` points at
        // `value.len()` readable bytes in this allocation for the duration
        // of the call. flags are 0 (create-or-replace).
        unsafe { libc::fsetxattr(fd, name.as_ptr(), value.as_ptr().cast(), value.len(), 0) }
    };

    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn getxattr_nofollow(path: &Path, name: &CStr) -> io::Result<Vec<u8>> {
    let path = path_c_string(path)?;
    let mut buf = vec![0u8; 256];
    let n = getxattr_nofollow_len(path.as_c_str(), name, buf.as_mut_ptr().cast(), buf.len())?;
    if n > buf.len() {
        buf.resize(n, 0);
        let n = getxattr_nofollow_len(path.as_c_str(), name, buf.as_mut_ptr().cast(), buf.len())?;
        buf.truncate(n);
        return Ok(buf);
    }
    buf.truncate(n);
    Ok(buf)
}

fn getxattr_nofollow_len(
    path: &CStr,
    name: &CStr,
    ptr: *mut libc::c_void,
    len: usize,
) -> io::Result<usize> {
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        let _ = (path, name, ptr, len);
        return Err(io::Error::from(io::ErrorKind::Unsupported));
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let n = {
        // SAFETY: `path` and `name` are NUL-terminated CStrs. `ptr`
        // addresses `len` writable bytes in the caller buffer for the
        // call. XATTR_NOFOLLOW prevents following a symlink destination.
        unsafe {
            libc::getxattr(
                path.as_ptr(),
                name.as_ptr(),
                ptr,
                len,
                0,
                libc::XATTR_NOFOLLOW,
            )
        }
    };
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let n = {
        // SAFETY: `path` and `name` are NUL-terminated CStrs. `ptr`
        // addresses `len` writable bytes in the caller buffer for the
        // call. `lgetxattr` does not follow a symlink destination.
        unsafe { libc::lgetxattr(path.as_ptr(), name.as_ptr(), ptr, len) }
    };

    if n < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(n as usize)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::owned::OwnershipToken;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;

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
        let facts =
            OwnedPublicationFacts::from_token_and_bytes(&token, b"payload-bytes", "image/png");
        set_owned_facts(file.as_raw_fd(), &facts).expect("fsetxattr must work on this filesystem");
        file.sync_all().unwrap();
        drop(file);

        assert!(!final_path.exists(), "final name must still be absent");
        let unpublished = read_owned_metadata(&tmp).unwrap();
        let parsed = OwnedPublicationFacts::from_object_metadata(&unpublished).unwrap();
        assert!(parsed.matches_token(&token));

        std::fs::hard_link(&tmp, &final_path).unwrap();
        let published = read_owned_metadata(&final_path).unwrap();
        assert_eq!(unpublished, published);
        assert_eq!(std::fs::read(&final_path).unwrap(), b"payload-bytes");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
