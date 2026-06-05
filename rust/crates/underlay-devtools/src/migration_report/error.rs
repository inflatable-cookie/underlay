#[derive(Debug)]
pub enum MigrationReportError {
    Io(std::io::Error),
    Json(serde_json::Error),
    InvalidInput(String),
}

impl std::fmt::Display for MigrationReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationReportError::Io(err) => write!(f, "{err}"),
            MigrationReportError::Json(err) => write!(f, "{err}"),
            MigrationReportError::InvalidInput(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for MigrationReportError {}
