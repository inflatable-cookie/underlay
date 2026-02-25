#[cfg(test)]
mod tests {
    use crate::extractors::extract_bearer;
    #[test]
    fn bearer_extraction_requires_bearer_prefix() {
        let value = http::HeaderValue::from_static("Token abc");
        assert!(extract_bearer(Some(&value)).is_none());
    }

    #[test]
    fn bearer_extraction_trims_token() {
        let value = http::HeaderValue::from_static("Bearer   abc123  ");
        assert_eq!(extract_bearer(Some(&value)).as_deref(), Some("abc123"));
    }

    #[test]
    fn bearer_extraction_fails_on_empty_token() {
        let value = http::HeaderValue::from_static("Bearer   ");
        assert!(extract_bearer(Some(&value)).is_none());
    }
}
