use super::AwsConfig;

#[test]
fn new_sets_region_and_no_endpoint() {
    let config = AwsConfig::new("us-east-1");
    assert_eq!(config.region, "us-east-1");
    assert_eq!(config.endpoint_url, None);
}

#[test]
fn with_endpoint_sets_endpoint_and_preserves_region() {
    let config = AwsConfig::new("eu-west-2").with_endpoint("http://localhost:4566");

    assert_eq!(config.region, "eu-west-2");
    assert_eq!(
        config.endpoint_url.as_deref(),
        Some("http://localhost:4566")
    );
}

#[test]
fn with_endpoint_overwrites_previous_endpoint() {
    let config = AwsConfig::new("us-west-2")
        .with_endpoint("http://first")
        .with_endpoint("http://second");

    assert_eq!(config.endpoint_url.as_deref(), Some("http://second"));
}

#[test]
fn with_static_credentials_sets_credentials_and_preserves_existing_fields() {
    let config = AwsConfig::new("eu-west-1")
        .with_endpoint("http://localhost:9000")
        .with_static_credentials("minioadmin", "minioadmin");

    assert_eq!(config.region, "eu-west-1");
    assert_eq!(config.endpoint_url.as_deref(), Some("http://localhost:9000"));
    let credentials = config.static_credentials.expect("static credentials");
    assert_eq!(credentials.access_key_id, "minioadmin");
    assert_eq!(credentials.secret_access_key, "minioadmin");
    assert_eq!(credentials.session_token, None);
}
