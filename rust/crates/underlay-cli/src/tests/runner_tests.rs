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