use std::path::Path;

use crate::DbPool;

#[derive(Debug, Clone, Copy)]
pub struct SqlDirOptions {
    /// Only files with this extension are executed.
    extension: &'static str,

    /// If true, execute files in lexicographic filename order.
    sort: bool,

    /// If true, split on ';' and run each non-empty statement.
    ///
    /// This is intended for simple seed scripts and is not suitable for procedural SQL
    /// blocks that contain semicolons (e.g. `DO $$ ... $$`).
    split_on_semicolon: bool,
}

impl Default for SqlDirOptions {
    fn default() -> Self {
        Self {
            extension: "sql",
            sort: true,
            split_on_semicolon: true,
        }
    }
}

impl SqlDirOptions {
    /// Create default SQL directory execution options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the file extension to execute.
    pub fn with_extension(mut self, extension: &'static str) -> Self {
        self.extension = extension;
        self
    }

    /// Set whether files should be executed in lexicographic filename order.
    pub fn with_sort(mut self, sort: bool) -> Self {
        self.sort = sort;
        self
    }

    /// Set whether SQL files should be split on semicolons.
    pub fn with_split_on_semicolon(mut self, split_on_semicolon: bool) -> Self {
        self.split_on_semicolon = split_on_semicolon;
        self
    }

    /// Return the file extension to execute.
    pub fn extension(&self) -> &'static str {
        self.extension
    }

    /// Return whether files should be sorted before execution.
    pub fn sort(&self) -> bool {
        self.sort
    }

    /// Return whether SQL files should be split on semicolons.
    pub fn split_on_semicolon(&self) -> bool {
        self.split_on_semicolon
    }
}

/// Execute all SQL files in `dir`.
///
/// This is primarily intended for dev seeds. Schema migrations should normally use
/// sqlx's migration system (`sqlx::migrate!`) so the binary can embed migrations.
pub async fn run_sql_dir(pool: &DbPool, dir: impl AsRef<Path>) -> Result<(), sqlx::Error> {
    run_sql_dir_with_options(pool, dir, SqlDirOptions::default()).await
}

pub async fn run_sql_dir_with_options(
    pool: &DbPool,
    dir: impl AsRef<Path>,
    options: SqlDirOptions,
) -> Result<(), sqlx::Error> {
    let dir = dir.as_ref();

    let mut entries = tokio::fs::read_dir(dir).await.map_err(sqlx::Error::Io)?;

    // A `read_dir` entry can fail independently (for example on a removed or
    // unreadable entry). Propagate that instead of silently skipping the file.
    let mut sql_files = Vec::new();
    while let Some(entry) = entries.next_entry().await.map_err(sqlx::Error::Io)? {
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext == options.extension())
        {
            sql_files.push(path);
        }
    }

    if options.sort() {
        sql_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    for path in sql_files {
        let sql = tokio::fs::read_to_string(&path)
            .await
            .map_err(sqlx::Error::Io)?;

        if options.split_on_semicolon() {
            for stmt in sql.split(';') {
                let trimmed = stmt.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // SQL failures stay real `sqlx::Error` values so callers keep
                // the database error kind instead of a wrapped IO error.
                sqlx::query(sqlx::AssertSqlSafe(trimmed))
                    .execute(pool)
                    .await?;
            }
        } else {
            // Run the whole file as one script so procedural blocks containing
            // semicolons (`DO $$ ... $$`, function bodies) stay intact.
            sqlx::raw_sql(sqlx::AssertSqlSafe(sql.as_str()))
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{run_sql_dir_with_options, SqlDirOptions};

    fn lazy_pool() -> crate::DbPool {
        // A lazy pool never connects, which is enough for tests that fail
        // before any statement executes.
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://underlay:underlay@127.0.0.1:1/underlay")
            .expect("lazy pool")
    }

    fn unique_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("underlay-sql-dir-{name}-{nanos}"))
    }

    #[test]
    fn sql_dir_options_builders_expose_read_only_values() {
        let options = SqlDirOptions::new()
            .with_extension("seed")
            .with_sort(false)
            .with_split_on_semicolon(false);

        assert_eq!(options.extension(), "seed");
        assert!(!options.sort());
        assert!(!options.split_on_semicolon());
    }

    #[tokio::test]
    async fn missing_directory_reports_io_error() {
        let dir = unique_dir("missing");
        let err = run_sql_dir_with_options(&lazy_pool(), &dir, SqlDirOptions::new())
            .await
            .expect_err("missing directory must fail");

        assert!(matches!(err, sqlx::Error::Io(_)));
    }

    #[tokio::test]
    async fn unreadable_sql_path_reports_io_error() {
        let dir = unique_dir("unreadable");
        // A directory with a `.sql` extension is selected like a script and
        // then fails to read as UTF-8 text.
        std::fs::create_dir_all(dir.join("nested.sql")).expect("create unreadable entry");
        let err = run_sql_dir_with_options(&lazy_pool(), &dir, SqlDirOptions::new())
            .await
            .expect_err("unreadable sql path must fail");

        assert!(matches!(err, sqlx::Error::Io(_)));
        std::fs::remove_dir_all(&dir).expect("remove fixture");
    }
}
