use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use toml::Value;

use crate::{discover_config_dir, ConfigError, ConfigStack};

/// `discover_config_dir` reads the process working directory, so every test
/// that moves it holds this lock. A poisoned lock only means another cwd test
/// panicked; the tests recover the guard rather than cascading.
static CWD_LOCK: Mutex<()> = Mutex::new(());

fn cwd_guard() -> std::sync::MutexGuard<'static, ()> {
    CWD_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

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
    let _guard = cwd_guard();
    let root = temp_config_dir("discover");
    let app = root.join("app");
    let config = root.join("config");
    fs::create_dir_all(&app).expect("create app dir");
    fs::create_dir_all(&config).expect("create config dir");
    fs::write(config.join("default.toml"), "[server]\nhost='127.0.0.1'\n").expect("write default");

    let discovered = from_working_dir(&app, || discover_config_dir(None));

    assert_eq!(discovered, PathBuf::from("../config"));
}

#[test]
fn discovers_parent_config_dir_inside_the_checkout_root() {
    let _guard = cwd_guard();
    let root = temp_config_dir("discover-in-root");
    let app = root.join("apps/api");
    fs::create_dir_all(&app).expect("create app dir");
    fs::create_dir_all(root.join(".git")).expect("mark checkout root");
    fs::create_dir_all(root.join("config")).expect("create config dir");
    fs::write(root.join("config/default.toml"), "[server]\n").expect("write default");

    let discovered = from_working_dir(&app, || discover_config_dir(None));

    // `../config` resolves to `apps/config`, which does not exist, so the
    // ancestor scan finds the checkout-root config inside the boundary.
    assert_eq!(discovered, root.join("config"));
}

#[test]
fn checkout_root_does_not_probe_config_outside_the_repository() {
    let _guard = cwd_guard();
    let outer = temp_config_dir("root-boundary");
    let checkout = outer.join("repo");
    fs::create_dir_all(checkout.join(".git")).expect("mark checkout root");
    fs::create_dir_all(outer.join("config")).expect("create outside config");
    fs::write(outer.join("config/default.toml"), "[server]\n").expect("write outside default");

    let discovered = from_working_dir(&checkout, || discover_config_dir(None));

    assert_ne!(discovered, PathBuf::from("../config"));
    assert_eq!(discovered, PathBuf::from("config"));
}

#[test]
fn boundary_check_is_lexical_and_component_wise() {
    let repo = Path::new("/repos/underlay");

    assert!(crate::discovery::resolves_inside_checkout_for_test(
        repo,
        repo,
        Path::new("config")
    ));
    assert!(crate::discovery::resolves_inside_checkout_for_test(
        &repo.join("apps/api"),
        repo,
        Path::new("../config")
    ));
    assert!(!crate::discovery::resolves_inside_checkout_for_test(
        repo,
        repo,
        Path::new("../config")
    ));
    assert!(!crate::discovery::resolves_inside_checkout_for_test(
        &repo.join("apps/api"),
        &repo.join("apps/api"),
        Path::new("../config")
    ));
    // A sibling whose name merely shares a prefix is not inside the checkout.
    assert!(!crate::discovery::resolves_inside_checkout_for_test(
        Path::new("/repos/underlay-other/app"),
        repo,
        Path::new("config")
    ));
}

/// Run `f` with the process working directory set to `dir`, restoring the
/// previous directory even if `f` panics.
fn from_working_dir<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
    struct RestoreCwd(PathBuf);

    impl Drop for RestoreCwd {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.0);
        }
    }

    let previous = env::current_dir().expect("current dir");
    env::set_current_dir(dir).expect("enter working dir");
    let _restore = RestoreCwd(previous);
    f()
}

fn temp_config_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    env::temp_dir().join(format!("underlay-config-{name}-{nanos}"))
}

#[test]
fn discovers_checkout_root_config_from_a_nested_app() {
    let root = temp_config_dir("ancestor");
    let app = root.join("apps/acme-api/crates/api");
    fs::create_dir_all(&app).expect("create nested app");
    fs::create_dir_all(root.join(".git")).expect("mark checkout root");
    fs::create_dir_all(root.join("config")).expect("create config dir");
    fs::write(root.join("config/default.toml"), "").expect("write default");

    assert_eq!(
        crate::discovery::ancestor_config_dir(&app),
        Some(root.join("config"))
    );
}

#[test]
fn ancestor_discovery_stops_at_the_checkout_root() {
    let outer = temp_config_dir("outside");
    let checkout = outer.join("repo");
    let app = checkout.join("apps/api");
    fs::create_dir_all(&app).expect("create nested app");
    fs::create_dir_all(checkout.join(".git")).expect("mark checkout root");
    fs::create_dir_all(outer.join("config")).expect("create outside config");
    fs::write(outer.join("config/default.toml"), "").expect("write outside default");

    assert_eq!(crate::discovery::ancestor_config_dir(&app), None);
}

#[test]
fn decode_error_exposes_toml_source() {
    let dir = temp_config_dir("decode-source");
    fs::create_dir_all(&dir).expect("create config dir");
    fs::write(
        dir.join("default.toml"),
        r#"
[server]
host = "127.0.0.1"
port = "not-a-number"

[auth]
issuer = "default"
"#,
    )
    .expect("write default");

    let err = ConfigStack::new(&dir)
        .with_environment("")
        .load::<AppConfig>()
        .expect_err("type mismatch must fail decode");

    assert!(matches!(err, ConfigError::Decode(_)));
    assert!(std::error::Error::source(&err).is_some());
}

#[test]
fn unreadable_optional_overlay_propagates_non_not_found_error() {
    let dir = temp_config_dir("unreadable-overlay");
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
    // A directory named `prod.toml` exists, so reading it fails with an I/O
    // error other than `NotFound` and must be surfaced rather than skipped.
    fs::create_dir_all(dir.join("prod.toml")).expect("create dir overlay");

    let err = ConfigStack::new(&dir)
        .with_environment("prod")
        .load::<AppConfig>()
        .expect_err("directory overlay must fail");
    assert!(matches!(err, ConfigError::Read { .. }));
}
