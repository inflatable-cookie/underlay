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

    let entries = std::fs::read_dir(dir).map_err(sqlx::Error::Io)?;

    let mut sql_files = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == options.extension())
        })
        .collect::<Vec<_>>();

    if options.sort() {
        sql_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    for path in sql_files {
        let sql = std::fs::read_to_string(&path).map_err(sqlx::Error::Io)?;

        if options.split_on_semicolon() {
            for stmt in sql.split(';') {
                let trimmed = stmt.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if let Err(err) = sqlx::query(sqlx::AssertSqlSafe(trimmed)).execute(pool).await {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("<unknown>");

                    return Err(sqlx::Error::Io(std::io::Error::other(format!(
                        "sql dir execution failed in {file_name}: {err}"
                    ))));
                }
            }
        } else if let Err(err) = sqlx::query(sqlx::AssertSqlSafe(sql)).execute(pool).await {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<unknown>");

            return Err(sqlx::Error::Io(std::io::Error::other(format!(
                "sql dir execution failed in {file_name}: {err}"
            ))));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SqlDirOptions;

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
}
