    use super::{require_env, DevtoolError};

    #[test]
    fn require_env_returns_value_when_present() {
        let value = require_env("PATH").expect("PATH should be present in test environment");
        assert!(!value.is_empty());
    }

    #[test]
    fn require_env_returns_missing_error_when_absent() {
        let err = require_env("UNDERLAY_DEVTOOLS_TEST_MISSING_ENV_SHOULD_NOT_EXIST")
            .expect_err("missing env var should return an error");
        match err {
            DevtoolError::MissingEnvVar { name } => {
                assert_eq!(name, "UNDERLAY_DEVTOOLS_TEST_MISSING_ENV_SHOULD_NOT_EXIST")
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }