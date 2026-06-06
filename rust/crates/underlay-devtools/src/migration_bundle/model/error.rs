#[derive(Debug)]
pub enum MigrationBundleError {
    Io(std::io::Error),
    Validation(String),
    InvalidInput(String),
}

impl std::fmt::Display for MigrationBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationBundleError::Io(err) => write!(f, "{err}"),
            MigrationBundleError::Validation(msg) => write!(f, "{msg}"),
            MigrationBundleError::InvalidInput(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for MigrationBundleError {}

impl From<std::io::Error> for MigrationBundleError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
