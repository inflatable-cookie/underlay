    use sqlx::migrate::{MigrateError, Migrator};

    #[test]
    fn migrator_implements_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<Migrator>();
        assert_sync::<Migrator>();
        assert_send::<MigrateError>();
        assert_sync::<MigrateError>();
    }

    #[test]
    fn migrate_error_type_converts_from_sqlx_error() {
        let sqlx_err: sqlx::Error = sqlx::Error::RowNotFound;
        let _migrate_err: MigrateError = sqlx_err.into();
    }