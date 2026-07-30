use super::*;

#[test]
fn environment_parse_and_display_cover_known_values() {
    assert_eq!(Environment::parse("local"), Environment::Local);
    assert_eq!(Environment::parse("development"), Environment::Dev);
    assert_eq!(Environment::parse("stage"), Environment::Staging);
    assert_eq!(Environment::parse("production"), Environment::Prod);
    assert_eq!(Environment::parse("test"), Environment::Test);
    assert_eq!(Environment::parse("unknown"), Environment::Prod);

    assert_eq!(Environment::Local.to_string(), "local");
    assert_eq!(Environment::Dev.to_string(), "dev");
    assert_eq!(Environment::Staging.to_string(), "staging");
    assert_eq!(Environment::Prod.to_string(), "prod");
    assert_eq!(Environment::Test.to_string(), "test");
}

#[test]
fn environment_helpers_select_expected_formats() {
    assert!(Environment::Local.is_development());
    assert!(Environment::Dev.is_development());
    assert!(!Environment::Prod.is_development());

    assert_eq!(Environment::Local.default_log_format(), LogFormat::Pretty);
    assert_eq!(Environment::Dev.default_log_format(), LogFormat::Pretty);
    assert_eq!(Environment::Staging.default_log_format(), LogFormat::Json);
    assert_eq!(Environment::Prod.default_log_format(), LogFormat::Json);
    assert_eq!(Environment::Test.default_log_format(), LogFormat::Json);
}

#[test]
fn observability_config_builders_apply_overrides() {
    let cfg = ObservabilityConfig::new()
        .with_level("debug")
        .with_json()
        .with_pretty()
        .with_format(LogFormat::Json)
        .with_environment(Environment::Prod);

    assert_eq!(cfg.level(), Some("debug"));
    assert_eq!(cfg.format(), LogFormat::Json);
    assert_eq!(cfg.environment(), Some(Environment::Prod));
}

#[test]
fn for_environment_uses_env_default_format() {
    let prod = ObservabilityConfig::for_environment(Environment::Prod);
    assert_eq!(prod.format(), LogFormat::Json);
    assert_eq!(prod.environment(), Some(Environment::Prod));
    assert_eq!(prod.level(), None);

    let local = ObservabilityConfig::for_environment(Environment::Local);
    assert_eq!(local.format(), LogFormat::Pretty);
    assert_eq!(local.environment(), Some(Environment::Local));
}
