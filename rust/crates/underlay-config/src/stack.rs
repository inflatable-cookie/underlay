use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use toml::Value;

use crate::merge::{merge_values, select_namespaced_or_legacy, set_dotted_value};
use crate::{ConfigError, DEFAULT_CONFIG_DIR, DEFAULT_ENVIRONMENT, DEFAULT_ENV_VAR};

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
