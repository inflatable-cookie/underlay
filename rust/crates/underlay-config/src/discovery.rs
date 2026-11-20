use std::env;
use std::path::{Component, Path, PathBuf};

use crate::DEFAULT_CONFIG_DIR;

/// Locate the config directory.
///
/// Order: the named environment variable, `./config`, then `../config` — but a
/// relative candidate is only probed when it still resolves inside the
/// checkout root (the nearest ancestor of the working directory containing
/// `.git`). After that, the nearest `config/` holding `default.toml` in an
/// ancestor of the working directory, stopping at the checkout root. Falls
/// back to `./config`.
pub fn discover_config_dir(config_dir_env_var: Option<&str>) -> PathBuf {
    if let Some(env_var) = config_dir_env_var {
        if let Ok(path) = env::var(env_var) {
            let path = path.trim();
            if !path.is_empty() {
                return PathBuf::from(path);
            }
        }
    }

    let cwd = env::current_dir().ok();
    let checkout_root = cwd.as_deref().and_then(find_checkout_root);

    for candidate in [DEFAULT_CONFIG_DIR, "../config"] {
        let path = PathBuf::from(candidate);
        if !resolves_inside_checkout(cwd.as_deref(), checkout_root.as_deref(), &path) {
            continue;
        }
        if path.join("default.toml").is_file() {
            return path;
        }
    }

    cwd.and_then(|cwd| ancestor_config_dir(&cwd))
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

/// Nearest ancestor of `start` (including `start`) that looks like a checkout
/// root. `.git` is a directory in a normal clone and a file in a worktree, so
/// existence is the test.
fn find_checkout_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(Path::to_path_buf)
}

/// Return `true` when `candidate`, resolved against `cwd`, stays inside the
/// checkout root. Without a known checkout root there is nothing to bound
/// against, so the historical relative probing is preserved.
fn resolves_inside_checkout(
    cwd: Option<&Path>,
    checkout_root: Option<&Path>,
    candidate: &Path,
) -> bool {
    let (Some(cwd), Some(checkout_root)) = (cwd, checkout_root) else {
        return true;
    };

    lexical_normalize(&cwd.join(candidate)).starts_with(lexical_normalize(checkout_root))
}

/// Resolve `.` and `..` components without touching the filesystem, so a
/// boundary check never depends on symlink or current-directory state.
fn lexical_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push("..");
                }
            }
            other => normalized.push(other.as_os_str()),
        }
    }

    normalized
}

#[cfg(test)]
pub(crate) fn resolves_inside_checkout_for_test(
    cwd: &Path,
    checkout_root: &Path,
    candidate: &Path,
) -> bool {
    resolves_inside_checkout(Some(cwd), Some(checkout_root), candidate)
}
