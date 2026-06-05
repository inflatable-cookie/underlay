use std::env;
use std::path::PathBuf;

use crate::DEFAULT_CONFIG_DIR;

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
