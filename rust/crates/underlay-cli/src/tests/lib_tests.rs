    use super::{parse_command, Command, PulseArgs, TaskInvocation};
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

    #[test]
    fn parse_runtime_task_passthrough() {
        let cmd = parse_command(vec![
            "snapshot".to_owned(),
            "--json".to_owned(),
            "--repo".to_owned(),
            ".".to_owned(),
        ])
        .expect("parse should succeed");
        assert_eq!(
            cmd,
            Command::Task(TaskInvocation {
                name: "snapshot".to_owned(),
                args: vec!["--json".to_owned(), "--repo".to_owned(), ".".to_owned()],
            })
        );
    }