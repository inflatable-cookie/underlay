//! Database URL classification helpers.

/// True when a database URL points at a local host: loopback
/// (`localhost`/`127.0.0.1`/`::1`) or a `*.test` container alias used by the
/// local dev stack.
///
/// Intended as a gate for destructive or seed operations so they can never
/// land on a remote database, regardless of how the environment resolved.
pub fn is_local_database_url(url: &str) -> bool {
    let after_at = url.rsplit('@').next().unwrap_or(url);
    // Bracketed IPv6 literals contain colons; extract them before splitting.
    let host = if let Some(bracketed) = after_at.strip_prefix('[') {
        bracketed.split(']').next().unwrap_or("")
    } else {
        after_at.split(['/', ':', '?']).next().unwrap_or("")
    };
    let host = host.trim().to_ascii_lowercase();

    host == "localhost" || host == "127.0.0.1" || host == "::1" || host.ends_with(".test")
}

#[cfg(test)]
mod tests {
    use super::is_local_database_url;

    #[test]
    fn local_database_urls_match() {
        assert!(is_local_database_url(
            "postgres://postgres:secret@127.0.0.1:5432/app"
        ));
        assert!(is_local_database_url(
            "postgres://postgres:secret@localhost/app"
        ));
        assert!(is_local_database_url(
            "postgres://postgres:secret@postgres.acme.test:5432/app"
        ));
        assert!(is_local_database_url(
            "mysql://root:secret@[::1]:3306/app"
        ));
    }

    #[test]
    fn remote_database_urls_do_not_match() {
        assert!(!is_local_database_url(
            "postgres://user:pass@db.example.com:5432/app"
        ));
        assert!(!is_local_database_url(
            "postgres://user:pass@10.0.0.5:5432/app"
        ));
        assert!(!is_local_database_url(
            "postgres://user:pass@staging.internal:5432/app"
        ));
    }
}
