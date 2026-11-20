//! Database URL classification helpers.

/// True when a database URL points at a local host: loopback
/// (`localhost`/`127.0.0.1`/`::1`) or a `*.test` container alias used by the
/// local dev stack.
///
/// Intended as a gate for destructive or seed operations so they can never
/// land on a remote database, regardless of how the environment resolved.
pub fn is_local_database_url(url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(url) else {
        return false;
    };

    match parsed.host() {
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        Some(url::Host::Domain(domain)) => {
            let domain = domain.to_ascii_lowercase();
            if let Ok(ip) = domain.parse::<std::net::IpAddr>() {
                return ip.is_loopback();
            }
            domain == "localhost" || domain.ends_with(".test")
        }
        None => false,
    }
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
        assert!(is_local_database_url("mysql://root:secret@[::1]:3306/app"));
        assert!(is_local_database_url("postgres://localhost:5432/app"));
    }

    #[test]
    fn remote_database_urls_do_not_match() {
        assert!(!is_local_database_url(
            "postgres://user:pass@db.example.com:5432/app"
        ));
        assert!(!is_local_database_url(
            "postgres://user:pass@prod.example.com:5432/app?application_name=x@localhost"
        ));
        assert!(!is_local_database_url(
            "postgres://user:pass@10.0.0.5:5432/app"
        ));
        assert!(!is_local_database_url(
            "postgres://user:pass@staging.internal:5432/app"
        ));
    }
}
