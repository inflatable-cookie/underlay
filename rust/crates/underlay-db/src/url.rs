//! Database URL classification helpers.

/// True when a database URL points at a local host: loopback
/// (`localhost`/`127.0.0.1`/`::1`) or a `*.test` container alias used by the
/// local dev stack, without any libpq `?host=` or `?hostaddr=` override
/// pointing elsewhere.
///
/// Intended as a gate for destructive or seed operations so they can never
/// land on a remote database, regardless of how the environment resolved.
pub fn is_local_database_url(url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(url) else {
        return false;
    };

    let authority_is_local = match parsed.host() {
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        Some(url::Host::Domain(domain)) => is_local_domain(domain),
        None => false,
    };

    if !authority_is_local {
        return false;
    }

    for (key, value) in parsed.query_pairs() {
        if key.eq_ignore_ascii_case("host") {
            if value.is_empty() || !value.split(',').all(is_local_host_token) {
                return false;
            }
        } else if key.eq_ignore_ascii_case("hostaddr")
            && (value.is_empty() || !value.split(',').all(is_local_hostaddr_token))
        {
            return false;
        }
    }

    true
}

fn is_local_domain(domain: &str) -> bool {
    let domain = domain.to_ascii_lowercase();
    if let Ok(ip) = domain.parse::<std::net::IpAddr>() {
        return ip.is_loopback();
    }
    domain == "localhost"
        || domain
            .strip_suffix(".test")
            .is_some_and(|prefix| !prefix.is_empty() && !prefix.ends_with('.'))
}

fn is_local_host_token(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let unbracketed = token
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or(token);
    if let Ok(ip) = unbracketed.parse::<std::net::IpAddr>() {
        return ip.is_loopback();
    }
    match url::Host::parse(token) {
        Ok(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Ok(url::Host::Ipv6(ip)) => ip.is_loopback(),
        Ok(url::Host::Domain(domain)) => is_local_domain(&domain),
        Err(_) => false,
    }
}

fn is_local_hostaddr_token(token: &str) -> bool {
    token
        .parse::<std::net::IpAddr>()
        .is_ok_and(|ip| ip.is_loopback())
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
        assert!(!is_local_database_url(
            "postgres://localhost/db?host=prod.example.com"
        ));
        assert!(!is_local_database_url(
            "postgres://localhost/db?host=localhost,prod.example.com"
        ));
        assert!(!is_local_database_url(
            "postgres://localhost/db?host=127.0.0.1&host=prod.example.com"
        ));
        assert!(!is_local_database_url("postgres://localhost/db?host="));
        assert!(!is_local_database_url(
            "postgres://localhost/db?hostaddr=203.0.113.10"
        ));
        assert!(!is_local_database_url(
            "postgres://postgres:secret@.test:5432/app"
        ));
        assert!(!is_local_database_url("postgres://localhost/db?host=.test"));
        assert!(is_local_database_url(
            "postgres://localhost/db?host=127.0.0.1"
        ));
        assert!(is_local_database_url(
            "postgres://localhost/db?host=postgres.acme.test"
        ));
    }
}
