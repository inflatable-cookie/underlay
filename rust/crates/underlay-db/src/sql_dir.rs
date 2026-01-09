use std::path::Path;

use crate::DbPool;

#[derive(Debug, Clone, Copy)]
pub struct SqlDirOptions {
    /// Only files with this extension are executed.
    pub extension: &'static str,

    /// If true, execute files in lexicographic filename order.
    pub sort: bool,

    /// If true, split on ';' and run each non-empty statement.
    ///
    /// This is intended for simple seed scripts and is not suitable for procedural SQL
    /// blocks that contain semicolons (e.g. `DO $$ ... $$`).
    pub split_on_semicolon: bool,
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
        .filter(|path| path.extension().is_some_and(|ext| ext == options.extension))
        .collect::<Vec<_>>();

    if options.sort {
        sql_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    for path in sql_files {
        let sql = std::fs::read_to_string(&path).map_err(sqlx::Error::Io)?;

        if options.split_on_semicolon {
            for stmt in sql.split(';') {
                let trimmed = stmt.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if let Err(err) = sqlx::query(trimmed).execute(pool).await {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("<unknown>");

                    return Err(sqlx::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("sql dir execution failed in {file_name}: {err}"),
                    )));
                }
            }
        } else {
            if let Err(err) = sqlx::query(&sql).execute(pool).await {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<unknown>");

                return Err(sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("sql dir execution failed in {file_name}: {err}"),
                )));
            }
        }
    }

    Ok(())
}
