use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use toml::Value;

use crate::{discover_config_dir, ConfigError, ConfigStack};

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct AppConfig {
    server: ServerConfig,
    auth: AuthConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct AuthConfig {
    issuer: String,
    audience: Option<String>,
}

#[test]
fn stacks_default_environment_local_and_explicit_overrides() {
    let dir = temp_config_dir("stack");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "default"
"#,
    )
    .expect("write default");
    fs::write(
        dir.join("uat.toml"),
        r#"
[server]
host = "0.0.0.0"

[auth]
issuer = "uat"
"#,
    )
    .expect("write uat");
    fs::write(
        dir.join("local.toml"),
        r#"
[server]
port = 4000
"#,
    )
    .expect("write local");

    let config: AppConfig = ConfigStack::new(&dir)
        .with_environment("uat")
        .with_optional_local_overlay("local")
        .with_env_override("auth.audience", Value::String("clients".to_owned()))
        .load()
        .expect("load config");

    assert_eq!(
        config,
        AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_owned(),
                port: 4000,
            },
            auth: AuthConfig {
                issuer: "uat".to_owned(),
                audience: Some("clients".to_owned()),
            },
        }
    );
}

#[test]
fn defaults_to_dev_environment_overlay() {
    let dir = temp_config_dir("default-dev");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "default"
"#,
    )
    .expect("write default");
    fs::write(
        dir.join("dev.toml"),
        r#"
[auth]
issuer = "dev"
"#,
    )
    .expect("write dev");

    let config: AppConfig = ConfigStack::new(&dir).load().expect("load config");

    assert_eq!(config.auth.issuer, "dev");
}

#[test]
fn missing_environment_overlay_is_allowed() {
    let dir = temp_config_dir("missing-env");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "default"
"#,
    )
    .expect("write default");

    let config: AppConfig = ConfigStack::new(&dir)
        .with_environment("production")
        .load()
        .expect("load config");

    assert_eq!(config.server.port, 3000);
    assert_eq!(config.auth.issuer, "default");
}

#[test]
fn rejects_path_like_environment_overlay_names() {
    let dir = temp_config_dir("path-like-env");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "default"
"#,
    )
    .expect("write default");

    let err = ConfigStack::new(&dir)
        .with_environment("../secrets")
        .load_value()
        .expect_err("reject path-like environment");

    assert!(matches!(
        err,
        ConfigError::InvalidOverlayName {
            reason: "name cannot contain path separators",
            ..
        }
    ));
}

#[test]
fn rejects_dot_local_overlay_names() {
    let dir = temp_config_dir("dot-local");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "default"
"#,
    )
    .expect("write default");

    let err = ConfigStack::new(&dir)
        .with_environment("")
        .with_optional_local_overlay("..")
        .load_value()
        .expect_err("reject dot local overlay");

    assert!(matches!(
        err,
        ConfigError::InvalidOverlayName {
            reason: "name cannot be a dot path component",
            ..
        }
    ));
}

#[test]
fn namespaced_config_overrides_legacy_root_values() {
    let dir = temp_config_dir("namespaced");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = 3000

[auth]
issuer = "legacy"

[my_app.server]
host = "0.0.0.0"

[my_app.auth]
issuer = "namespaced"
audience = "clients"
"#,
    )
    .expect("write default");

    let config: AppConfig = ConfigStack::new(&dir)
        .load_namespaced_or_legacy("my_app")
        .expect("load config");

    assert_eq!(
        config,
        AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_owned(),
                port: 3000,
            },
            auth: AuthConfig {
                issuer: "namespaced".to_owned(),
                audience: Some("clients".to_owned()),
            },
        }
    );
}

#[test]
fn discovers_parent_config_dir_when_local_dir_missing() {
    let root = temp_config_dir("discover");
    let app = root.join("app");
    let config = root.join("config");
    fs::create_dir_all(&app).expect("create app dir");
    fs::create_dir_all(&config).expect("create config dir");
    fs::write(config.join("default.toml"), "[server]\nhost='127.0.0.1'\n").expect("write default");

    let previous_dir = env::current_dir().expect("current dir");
    env::set_current_dir(&app).expect("enter app dir");
    let discovered = discover_config_dir(None);
    env::set_current_dir(previous_dir).expect("restore dir");

    assert_eq!(discovered, PathBuf::from("../config"));
}

fn temp_config_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    env::temp_dir().join(format!("underlay-config-{name}-{nanos}"))
}
