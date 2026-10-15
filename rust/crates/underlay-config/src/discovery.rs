use std::env;
use std::path::{Path, PathBuf};

use crate::DEFAULT_CONFIG_DIR;

/// Locate the config directory.
///
/// Order: the named environment variable, `./config`, `../config`, then the
/// nearest `config/` holding `default.toml` in an ancestor of the working
/// directory, stopping at the checkout root (the first ancestor containing
/// `.git`). Falls back to `./config`.
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

    env::current_dir()
        .ok()
        .and_then(|cwd| ancestor_config_dir(&cwd))
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_DIR))
}

/// Nearest `config/default.toml` at or above `start`, bounded by the checkout
/// root so a nested app never picks up configuration from outside its repo.
pub(crate) fn ancestor_config_dir(start: &Path) -> Option<PathBuf> {
    for dir in start.ancestors() {
        let candidate = dir.join(DEFAULT_CONFIG_DIR);
        if candidate.join("default.toml").is_file() {
            return Some(candidate);
        }
        if dir.join(".git").exists() {
            return None;
        }
    }
    None
}
