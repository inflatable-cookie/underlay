use crate::checkers::pulse::PulseChecker;
use crate::checkers::{Checker, CheckerError, RunnerContext};
use crate::resolver::{resolve_target_root, ResolveError};
use crate::PulseArgs;

#[derive(Debug)]
pub enum RunnerError {
    Cwd(std::io::Error),
    Resolve(ResolveError),
    Checker(CheckerError),
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerError::Cwd(err) => write!(f, "failed to resolve current directory: {err}"),
            RunnerError::Resolve(err) => write!(f, "{err}"),
            RunnerError::Checker(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for RunnerError {}

impl From<CheckerError> for RunnerError {
    fn from(value: CheckerError) -> Self {
        Self::Checker(value)
    }
}

impl From<ResolveError> for RunnerError {
    fn from(value: ResolveError) -> Self {
        Self::Resolve(value)
    }
}

pub fn run_pulse(args: PulseArgs) -> Result<String, RunnerError> {
    let PulseArgs {
        repo_override,
        verbose_root,
    } = args;
    let cwd = std::env::current_dir().map_err(RunnerError::Cwd)?;
    let resolved = resolve_target_root(cwd.clone(), repo_override)?;

    let checker = PulseChecker::default();
    let ctx = RunnerContext {
        target_repo: resolved.resolved_root.clone(),
        cwd,
        resolution_mode: resolved.resolution_mode,
        resolution_evidence: resolved.evidence,
        resolution_warnings: resolved.warnings,
    };

    let collected = checker.collect(&ctx)?;
    let evaluated = checker.evaluate(collected)?;
    let report = checker.render(evaluated).map_err(RunnerError::from)?;

    if verbose_root {
        let mut trace = String::new();
        trace.push_str("# Root Resolution\n\n");
        trace.push_str(&format!(
            "- resolved-root: {}\n",
            resolved.resolved_root.display()
        ));
        trace.push_str(&format!("- mode: {:?}\n", resolved.resolution_mode));
        if !ctx.resolution_evidence.is_empty() {
            trace.push_str("- evidence:\n");
            for item in &ctx.resolution_evidence {
                trace.push_str(&format!("  - {}\n", item));
            }
        }
        if !ctx.resolution_warnings.is_empty() {
            trace.push_str("- warnings:\n");
            for item in &ctx.resolution_warnings {
                trace.push_str(&format!("  - {}\n", item));
            }
        }
        trace.push('\n');
        trace.push_str(&report);
        return Ok(trace);
    }

    Ok(report)
}
