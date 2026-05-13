//! Layered TOML configuration helpers for Underlay applications.
//!
//! The loader owns file stacking only. Apps still own their typed config
//! structs, validation, and explicit env override allowlists.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use toml::Value;

pub const DEFAULT_CONFIG_DIR: &str = "config";
pub const DEFAULT_ENVIRONMENT: &str = "dev";
pub const DEFAULT_ENV_VAR: &str = "ENVIRONMENT_NAME";

pub fn discover_config_dir(config_dir_env_var: Option<&str>) -> PathBuf {
    if let Some(env_var) = config_dir_env_var {
        if let Ok(path) = env::var(env_var) {
            let path = path.trim();
            if !path.is_empty() {
                return PathBuf::from(path);
            }
        }
    }

    for candidate in [DEFAULT_CONFIG_DIR, "../config"] {
        let path = PathBuf::from(candidate);
        if path.join("default.toml").is_file() {
            return path;
        }
    }

    PathBuf::from(DEFAULT_CONFIG_DIR)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigStack {
    config_dir: PathBuf,
    environment: Option<String>,
    local_overlay: Option<String>,
    env_overrides: Vec<(String, Value)>,
}

impl ConfigStack {
    pub fn new(config_dir: impl Into<PathBuf>) -> Self {
        Self {
            config_dir: config_dir.into(),
            environment: Some(DEFAULT_ENVIRONMENT.to_owned()),
            local_overlay: None,
            env_overrides: Vec::new(),
        }
    }

    pub fn from_project_root(root: impl AsRef<Path>) -> Self {
        Self::new(root.as_ref().join(DEFAULT_CONFIG_DIR))
    }

    pub fn with_environment(mut self, environment: impl Into<String>) -> Self {
        let environment = environment.into();
        self.environment = if environment.trim().is_empty() {
            None
        } else {
            Some(environment)
        };
        self
    }

    pub fn with_environment_from_env(mut self) -> Self {
        self.environment = Some(
            env::var(DEFAULT_ENV_VAR)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| DEFAULT_ENVIRONMENT.to_owned()),
        );
        self
    }

    pub fn with_optional_local_overlay(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.local_overlay = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }

    pub fn with_env_override(mut self, dotted_key: impl Into<String>, value: Value) -> Self {
        self.env_overrides.push((dotted_key.into(), value));
        self
    }

    pub fn load<T>(&self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
    {
        let value = self.load_value()?;
        value.try_into().map_err(ConfigError::Decode)
    }

    pub fn load_namespaced_or_legacy<T>(&self, namespace: &str) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
    {
        let value = select_namespaced_or_legacy(self.load_value()?, namespace);
        value.try_into().map_err(ConfigError::Decode)
    }

    pub fn load_value(&self) -> Result<Value, ConfigError> {
        let mut merged = Value::Table(Default::default());

        self.merge_required_file(&mut merged, "default")?;

        if let Some(environment) = &self.environment {
            self.merge_optional_file(&mut merged, environment)?;
        }

        if let Some(local_overlay) = &self.local_overlay {
            self.merge_optional_file(&mut merged, local_overlay)?;
        }

        for (key, value) in &self.env_overrides {
            set_dotted_value(&mut merged, key, value.clone())?;
        }

        Ok(merged)
    }

    fn merge_required_file(&self, merged: &mut Value, name: &str) -> Result<(), ConfigError> {
        let path = self.config_dir.join(format!("{name}.toml"));
        let next = read_toml_file(&path)?;
        merge_values(merged, next);
        Ok(())
    }

    fn merge_optional_file(&self, merged: &mut Value, name: &str) -> Result<(), ConfigError> {
        let path = self.config_dir.join(format!("{name}.toml"));
        if !path.exists() {
            return Ok(());
        }

        let next = read_toml_file(&path)?;
        merge_values(merged, next);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file `{path}`: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse config file `{path}`: {source}")]
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    #[error("failed to decode typed config: {0}")]
    Decode(toml::de::Error),
    #[error("config override key `{0}` is empty")]
    EmptyOverrideKey(String),
    #[error("config override key `{0}` cannot descend through non-table value")]
    NonTableOverride(String),
}

fn read_toml_file(path: &Path) -> Result<Value, ConfigError> {
    let raw = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_path_buf(),
        source,
    })?;

    raw.parse::<Value>().map_err(|source| ConfigError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

fn merge_values(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Table(base), Value::Table(overlay)) => {
            for (key, value) in overlay {
                match base.get_mut(&key) {
                    Some(existing) => merge_values(existing, value),
                    None => {
                        base.insert(key, value);
                    }
                }
            }
        }
        (base, overlay) => {
            *base = overlay;
        }
    }
}

fn select_namespaced_or_legacy(value: Value, namespace: &str) -> Value {
    let Value::Table(mut root) = value else {
        return value;
    };

    let namespaced = root.remove(namespace);
    let mut selected = Value::Table(root);
    if let Some(namespaced) = namespaced {
        merge_values(&mut selected, namespaced);
    }

    selected
}

fn set_dotted_value(root: &mut Value, dotted_key: &str, value: Value) -> Result<(), ConfigError> {
    let parts: Vec<&str> = dotted_key.split('.').collect();
    if parts.iter().any(|part| part.trim().is_empty()) {
        return Err(ConfigError::EmptyOverrideKey(dotted_key.to_owned()));
    }

    let mut current = root;
    for part in &parts[..parts.len() - 1] {
        let Value::Table(table) = current else {
            return Err(ConfigError::NonTableOverride(dotted_key.to_owned()));
        };
        current = table
            .entry((*part).to_owned())
            .or_insert_with(|| Value::Table(Default::default()));
    }

    let Value::Table(table) = current else {
        return Err(ConfigError::NonTableOverride(dotted_key.to_owned()));
    };
    table.insert(parts[parts.len() - 1].to_owned(), value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde::Deserialize;

    use super::*;

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
        fs::write(config.join("default.toml"), "[server]\nhost='127.0.0.1'\n")
            .expect("write default");

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
}
