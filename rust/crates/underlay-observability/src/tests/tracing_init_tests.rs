use super::*;

#[test]
fn environment_parse_and_display_cover_known_values() {
    assert_eq!(Environment::parse("local"), Environment::Local);
    assert_eq!(Environment::parse("effigy"), Environment::Effigy);
    assert_eq!(Environment::parse("EFFIGY"), Environment::Effigy);
    assert_eq!(Environment::parse("development"), Environment::Dev);
    assert_eq!(Environment::parse("stage"), Environment::Staging);
    assert_eq!(Environment::parse("production"), Environment::Prod);
    assert_eq!(Environment::parse("test"), Environment::Test);
    assert_eq!(Environment::parse("unknown"), Environment::Prod);

    assert_eq!(Environment::Local.to_string(), "local");
    assert_eq!(Environment::Effigy.to_string(), "effigy");
    assert_eq!(Environment::Dev.to_string(), "dev");
    assert_eq!(Environment::Staging.to_string(), "staging");
    assert_eq!(Environment::Prod.to_string(), "prod");
    assert_eq!(Environment::Test.to_string(), "test");
}

#[test]
fn environment_resolve_prefers_primary_var_and_fails_closed() {
    // Unique var names so parallel tests cannot interfere.
    const PRIMARY: &str = "UNDERLAY_TEST_RESOLVE_PRIMARY";
    const LEGACY: &str = "UNDERLAY_TEST_RESOLVE_LEGACY";

    // Unset everywhere fails closed to Prod.
    std::env::remove_var(PRIMARY);
    std::env::remove_var(LEGACY);
    assert_eq!(Environment::resolve(PRIMARY, Some(LEGACY)), Environment::Prod);

    // Legacy var is honored when primary is unset.
    std::env::set_var(LEGACY, "effigy");
    assert_eq!(Environment::resolve(PRIMARY, Some(LEGACY)), Environment::Effigy);

    // Primary wins over legacy; unknown values fail closed.
    std::env::set_var(PRIMARY, "staging");
    assert_eq!(Environment::resolve(PRIMARY, Some(LEGACY)), Environment::Staging);
    std::env::set_var(PRIMARY, "bogus");
    assert_eq!(Environment::resolve(PRIMARY, Some(LEGACY)), Environment::Prod);

    std::env::remove_var(PRIMARY);
    std::env::remove_var(LEGACY);
}

#[test]
fn environment_resolve_name_keeps_raw_overlay_names() {
    const PRIMARY: &str = "UNDERLAY_TEST_RESOLVE_NAME_PRIMARY";
    const LEGACY: &str = "UNDERLAY_TEST_RESOLVE_NAME_LEGACY";

    std::env::remove_var(PRIMARY);
    std::env::remove_var(LEGACY);
    assert_eq!(Environment::resolve_name(PRIMARY, Some(LEGACY)), None);

    // Arbitrary overlay names pass through un-normalized (uat is not an
    // Environment variant and must not become "prod").
    std::env::set_var(LEGACY, "uat");
    assert_eq!(
        Environment::resolve_name(PRIMARY, Some(LEGACY)),
        Some("uat".to_string())
    );

    std::env::set_var(PRIMARY, " effigy ");
    assert_eq!(
        Environment::resolve_name(PRIMARY, Some(LEGACY)),
        Some("effigy".to_string())
    );

    std::env::remove_var(PRIMARY);
    std::env::remove_var(LEGACY);
}

#[test]
fn environment_helpers_select_expected_formats() {
    assert!(Environment::Local.is_development());
    assert!(Environment::Effigy.is_development());
    assert!(Environment::Dev.is_development());
    assert!(!Environment::Prod.is_development());

    assert!(Environment::Local.is_local_dev());
    assert!(Environment::Effigy.is_local_dev());
    assert!(Environment::Test.is_local_dev());
    assert!(!Environment::Dev.is_local_dev());
    assert!(!Environment::Staging.is_local_dev());
    assert!(!Environment::Prod.is_local_dev());

    assert_eq!(Environment::Local.default_log_format(), LogFormat::Pretty);
    assert_eq!(Environment::Effigy.default_log_format(), LogFormat::Pretty);
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
