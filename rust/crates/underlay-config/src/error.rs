use std::path::PathBuf;

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
