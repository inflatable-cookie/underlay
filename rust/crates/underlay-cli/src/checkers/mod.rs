pub mod pulse;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerContext {
    pub target_repo: PathBuf,
    pub cwd: PathBuf,
    pub resolution_mode: ResolutionMode,
    pub resolution_evidence: Vec<String>,
    pub resolution_warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    Explicit,
    AutoNearest,
    AutoPromoted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseReport {
    pub repo: String,
    pub evidence: Vec<String>,
    pub risk: Vec<String>,
    pub next_action: Vec<String>,
    pub owner: String,
    pub eta: String,
}

pub trait Checker {
    type Collected;
    type Evaluated;

    fn id(&self) -> &'static str;
    fn collect(&self, ctx: &RunnerContext) -> Result<Self::Collected, CheckerError>;
    fn evaluate(&self, collected: Self::Collected) -> Result<Self::Evaluated, CheckerError>;
    fn render(&self, evaluated: Self::Evaluated) -> Result<String, CheckerError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckerError {
    Io(String),
}

impl std::fmt::Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckerError::Io(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for CheckerError {}
