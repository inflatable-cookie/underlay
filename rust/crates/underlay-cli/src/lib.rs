pub mod checkers;
pub mod resolver;
pub mod runner;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Pulse(PulseArgs),
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseArgs {
    pub repo_override: Option<PathBuf>,
    pub verbose_root: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliParseError {
    MissingRepoValue,
    UnknownArgument(String),
    UnknownCommand(String),
}

impl std::fmt::Display for CliParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliParseError::MissingRepoValue => write!(f, "--repo requires a value"),
            CliParseError::UnknownArgument(arg) => write!(f, "unknown argument: {arg}"),
            CliParseError::UnknownCommand(cmd) => write!(f, "unknown command: {cmd}"),
        }
    }
}

impl std::error::Error for CliParseError {}

pub fn parse_command<I>(args: I) -> Result<Command, CliParseError>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(cmd) = args.next() else {
        return Ok(Command::Help);
    };

    if cmd == "--help" || cmd == "-h" {
        return Ok(Command::Help);
    }

    match cmd.as_str() {
        "pulse" => {
            let mut repo_override: Option<PathBuf> = None;
            let mut verbose_root = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--repo" => {
                        let Some(path) = args.next() else {
                            return Err(CliParseError::MissingRepoValue);
                        };
                        repo_override = Some(PathBuf::from(path));
                    }
                    "--verbose-root" => {
                        verbose_root = true;
                    }
                    "--help" | "-h" => return Ok(Command::Help),
                    other => return Err(CliParseError::UnknownArgument(other.to_owned())),
                }
            }

            Ok(Command::Pulse(PulseArgs {
                repo_override,
                verbose_root,
            }))
        }
        other => Err(CliParseError::UnknownCommand(other.to_owned())),
    }
}

pub fn print_usage() {
    eprintln!(
        "underlay\n\nUSAGE:\n  underlay pulse [--repo <PATH>] [--verbose-root]\n\nCOMMANDS:\n  pulse             Run the repo pulse checker\n\nOPTIONS:\n  --repo <PATH>     Override target repository path\n  --verbose-root    Print root resolution trace\n  -h, --help        Print help\n"
    );
}

#[cfg(test)]
mod tests {
    use super::{parse_command, Command, PulseArgs};
    use std::path::PathBuf;

    #[test]
    fn parse_defaults_to_help_without_command() {
        let cmd = parse_command(Vec::<String>::new()).expect("parse should succeed");
        assert_eq!(cmd, Command::Help);
    }

    #[test]
    fn parse_pulse_with_repo_override() {
        let cmd = parse_command(vec![
            "pulse".to_owned(),
            "--repo".to_owned(),
            "/tmp/repo".to_owned(),
        ])
        .expect("parse should succeed");
        assert_eq!(
            cmd,
            Command::Pulse(PulseArgs {
                repo_override: Some(PathBuf::from("/tmp/repo")),
                verbose_root: false,
            })
        );
    }

    #[test]
    fn parse_pulse_with_verbose_root() {
        let cmd = parse_command(vec!["pulse".to_owned(), "--verbose-root".to_owned()])
            .expect("parse should succeed");
        assert_eq!(
            cmd,
            Command::Pulse(PulseArgs {
                repo_override: None,
                verbose_root: true,
            })
        );
    }
}
