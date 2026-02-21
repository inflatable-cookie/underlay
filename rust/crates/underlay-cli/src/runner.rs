use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use crate::resolver::{resolve_target_root, ResolveError};
use crate::tasks::pulse::PulseTask;
use crate::tasks::{Task, TaskContext, TaskError};
use crate::{Command, PulseArgs, TaskInvocation};

#[derive(Debug)]
pub enum RunnerError {
    Cwd(std::io::Error),
    Resolve(ResolveError),
    Task(TaskError),
    TaskInvocation(String),
    TaskManifestMissing { path: PathBuf },
    TaskManifestRead { path: PathBuf, error: std::io::Error },
    TaskManifestParse { path: PathBuf, error: toml::de::Error },
    TaskNotFound {
        name: String,
        path: PathBuf,
    },
    TaskCommandLaunch { command: String, error: std::io::Error },
    TaskCommandFailure {
        command: String,
        code: Option<i32>,
        stdout: String,
        stderr: String,
    },
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerError::Cwd(err) => write!(f, "failed to resolve current directory: {err}"),
            RunnerError::Resolve(err) => write!(f, "{err}"),
            RunnerError::Task(err) => write!(f, "{err}"),
            RunnerError::TaskInvocation(msg) => write!(f, "{msg}"),
            RunnerError::TaskManifestMissing { path } => write!(
                f,
                "task manifest not found: {} (expected underlay.tasks.toml at project root)",
                path.display()
            ),
            RunnerError::TaskManifestRead { path, error } => {
                write!(f, "failed to read {}: {error}", path.display())
            }
            RunnerError::TaskManifestParse { path, error } => {
                write!(f, "failed to parse {}: {error}", path.display())
            }
            RunnerError::TaskNotFound { name, path } => write!(
                f,
                "task `{name}` is not defined in {}",
                path.display()
            ),
            RunnerError::TaskCommandLaunch { command, error } => {
                write!(f, "failed to launch task command `{command}`: {error}")
            }
            RunnerError::TaskCommandFailure {
                command,
                code,
                stdout,
                stderr,
            } => write!(
                f,
                "task command failed `{command}` (code={:?})\nstdout:\n{}\nstderr:\n{}",
                code, stdout, stderr
            ),
        }
    }
}

impl std::error::Error for RunnerError {}

impl From<TaskError> for RunnerError {
    fn from(value: TaskError) -> Self {
        Self::Task(value)
    }
}

impl From<ResolveError> for RunnerError {
    fn from(value: ResolveError) -> Self {
        Self::Resolve(value)
    }
}

#[derive(Debug, serde::Deserialize)]
struct TaskManifest {
    tasks: BTreeMap<String, ManifestTask>,
}

#[derive(Debug, serde::Deserialize)]
struct ManifestTask {
    run: String,
}

pub fn run_command(cmd: Command) -> Result<String, RunnerError> {
    match cmd {
        Command::Help => Ok(String::new()),
        Command::Pulse(args) => run_pulse(args),
        Command::Task(task) => run_manifest_task(&task),
    }
}

pub fn run_pulse(args: PulseArgs) -> Result<String, RunnerError> {
    let PulseArgs {
        repo_override,
        verbose_root,
    } = args;
    let cwd = std::env::current_dir().map_err(RunnerError::Cwd)?;
    let resolved = resolve_target_root(cwd.clone(), repo_override)?;

    let task = PulseTask::default();
    let ctx = TaskContext {
        target_repo: resolved.resolved_root.clone(),
        cwd,
        resolution_mode: resolved.resolution_mode,
        resolution_evidence: resolved.evidence,
        resolution_warnings: resolved.warnings,
    };

    let collected = task.collect(&ctx)?;
    let evaluated = task.evaluate(collected)?;
    let report = task.render(evaluated).map_err(RunnerError::from)?;

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

fn run_manifest_task(task: &TaskInvocation) -> Result<String, RunnerError> {
    let cwd = std::env::current_dir().map_err(RunnerError::Cwd)?;
    let (repo_override, passthrough_args) = parse_repo_override(&task.args)?;
    let resolved = resolve_target_root(cwd, repo_override)?;

    let manifest_path = resolved.resolved_root.join("underlay.tasks.toml");
    if !manifest_path.exists() {
        return Err(RunnerError::TaskManifestMissing {
            path: manifest_path,
        });
    }

    let manifest_src = fs::read_to_string(&manifest_path).map_err(|error| {
        RunnerError::TaskManifestRead {
            path: manifest_path.clone(),
            error,
        }
    })?;
    let manifest: TaskManifest =
        toml::from_str(&manifest_src).map_err(|error| RunnerError::TaskManifestParse {
            path: manifest_path.clone(),
            error,
        })?;

    let Some(definition) = manifest.tasks.get(&task.name) else {
        return Err(RunnerError::TaskNotFound {
            name: task.name.clone(),
            path: manifest_path,
        });
    };

    let args_rendered = passthrough_args
        .iter()
        .map(|arg| shell_quote(arg))
        .collect::<Vec<String>>()
        .join(" ");
    let repo_rendered = shell_quote(&resolved.resolved_root.display().to_string());

    let command = definition
        .run
        .replace("{repo}", &repo_rendered)
        .replace("{args}", &args_rendered);

    let output = ProcessCommand::new("sh")
        .arg("-lc")
        .arg(&command)
        .current_dir(&resolved.resolved_root)
        .output()
        .map_err(|error| RunnerError::TaskCommandLaunch {
            command: command.clone(),
            error,
        })?;

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }

    Err(RunnerError::TaskCommandFailure {
        command,
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

fn parse_repo_override(args: &[String]) -> Result<(Option<PathBuf>, Vec<String>), RunnerError> {
    let mut repo: Option<PathBuf> = None;
    let mut passthrough: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--repo" {
            let Some(value) = args.get(i + 1) else {
                return Err(RunnerError::TaskInvocation(
                    "task argument --repo requires a value".to_owned(),
                ));
            };
            repo = Some(PathBuf::from(value));
            i += 2;
            continue;
        }
        passthrough.push(arg.clone());
        i += 1;
    }
    Ok((repo, passthrough))
}

fn shell_quote(raw: &str) -> String {
    if raw.is_empty() {
        return "''".to_owned();
    }
    let escaped = raw.replace('\'', "'\"'\"'");
    format!("'{escaped}'")
}

#[cfg(test)]
mod tests {
    use super::parse_repo_override;
    use std::path::PathBuf;

    #[test]
    fn parse_repo_override_extracts_repo_and_passthrough() {
        let args = vec![
            "--repo".to_owned(),
            "/tmp/x".to_owned(),
            "--flag".to_owned(),
            "abc".to_owned(),
        ];
        let (repo, passthrough) = parse_repo_override(&args).expect("parse");
        assert_eq!(repo, Some(PathBuf::from("/tmp/x")));
        assert_eq!(passthrough, vec!["--flag".to_owned(), "abc".to_owned()]);
    }
}
