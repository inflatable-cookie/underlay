use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SyncMigrationsReport {
    pub written: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum SyncMigrationsError {
    TargetMissing(PathBuf),
    TargetNotDir(PathBuf),
    SourceMissing(PathBuf),
    InvalidMigrationFilename(String),
    Io(std::io::Error),
    ContentMismatch { path: PathBuf },
}

impl std::fmt::Display for SyncMigrationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncMigrationsError::TargetMissing(p) => {
                write!(f, "target directory does not exist: {}", p.display())
            }
            SyncMigrationsError::TargetNotDir(p) => {
                write!(f, "target is not a directory: {}", p.display())
            }
            SyncMigrationsError::SourceMissing(p) => {
                write!(f, "source directory does not exist: {}", p.display())
            }
            SyncMigrationsError::InvalidMigrationFilename(name) => {
                write!(
                    f,
                    "invalid migration filename (expected leading digits): {name}"
                )
            }
            SyncMigrationsError::Io(e) => write!(f, "{e}"),
            SyncMigrationsError::ContentMismatch { path } => {
                write!(f, "existing migration differs: {}", path.display())
            }
        }
    }
}

impl std::error::Error for SyncMigrationsError {}

impl From<std::io::Error> for SyncMigrationsError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone, Copy)]
struct MigrationSource {
    slug: &'static str,
    crate_id: u32,
    /// Path relative to underlay-devtools crate root.
    migrations_rel: &'static str,
}

// Reserve high version space for Underlay-synced migrations.
//
// The generated version number is: (crate_id * 100_000_000) + original_version
// which yields a 12-digit numeric prefix and ensures stable ordering per crate.
const SOURCES: &[MigrationSource] = &[MigrationSource {
    slug: "underlay_auth",
    crate_id: 9000,
    migrations_rel: "../underlay-auth/migrations",
}];

pub fn sync_migrations(
    target_dir: impl AsRef<Path>,
    dry_run: bool,
) -> Result<SyncMigrationsReport, SyncMigrationsError> {
    let target_dir = target_dir.as_ref();

    if !target_dir.exists() {
        return Err(SyncMigrationsError::TargetMissing(target_dir.to_path_buf()));
    }
    if !target_dir.is_dir() {
        return Err(SyncMigrationsError::TargetNotDir(target_dir.to_path_buf()));
    }

    let mut report = SyncMigrationsReport {
        written: Vec::new(),
        skipped: Vec::new(),
    };

    let devtools_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    for source in SOURCES {
        let src_dir = devtools_dir.join(source.migrations_rel);
        if !src_dir.exists() {
            return Err(SyncMigrationsError::SourceMissing(src_dir));
        }

        let mut entries = std::fs::read_dir(&src_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
            .collect::<Vec<_>>();
        entries.sort();

        for src_path in entries {
            let file_name = src_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| {
                    SyncMigrationsError::InvalidMigrationFilename("<non-utf8>".into())
                })?;

            let orig_version = parse_leading_version(file_name)?;
            let new_version = u64::from(source.crate_id) * 100_000_000 + orig_version;

            let target_name = format!("{new_version:012}__{}__{file_name}", source.slug);
            let target_path = target_dir.join(target_name);

            let src_contents = std::fs::read_to_string(&src_path)?;

            if target_path.exists() {
                let existing = std::fs::read_to_string(&target_path)?;
                if existing == src_contents {
                    report.skipped.push(target_path);
                    continue;
                }
                return Err(SyncMigrationsError::ContentMismatch { path: target_path });
            }

            if dry_run {
                report.written.push(target_path);
                continue;
            }

            std::fs::write(&target_path, src_contents)?;
            report.written.push(target_path);
        }
    }

    Ok(report)
}

fn parse_leading_version(file_name: &str) -> Result<u64, SyncMigrationsError> {
    let digits = file_name
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    if digits.is_empty() {
        return Err(SyncMigrationsError::InvalidMigrationFilename(
            file_name.to_string(),
        ));
    }

    digits
        .parse::<u64>()
        .map_err(|_| SyncMigrationsError::InvalidMigrationFilename(file_name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{parse_leading_version, sync_migrations, SyncMigrationsError};
    use std::path::PathBuf;

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
        std::fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    #[test]
    fn parse_leading_version_accepts_numeric_prefix() {
        let version = parse_leading_version("0002_create_auth_tables.sql")
            .expect("version should parse from filename prefix");
        assert_eq!(version, 2);
    }

    #[test]
    fn parse_leading_version_rejects_missing_prefix() {
        let err = parse_leading_version("create_auth_tables.sql")
            .expect_err("filename without leading digits should fail");
        match err {
            SyncMigrationsError::InvalidMigrationFilename(name) => {
                assert_eq!(name, "create_auth_tables.sql")
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn sync_migrations_errors_when_target_missing() {
        let path = temp_dir("underlay_sync_missing_target");
        std::fs::remove_dir_all(&path).expect("temp dir should be removed");

        let err = sync_migrations(&path, true).expect_err("missing target should fail");
        match err {
            SyncMigrationsError::TargetMissing(p) => assert_eq!(p, path),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn sync_migrations_writes_then_skips_when_unchanged() {
        let path = temp_dir("underlay_sync_write_skip");

        let first = sync_migrations(&path, false).expect("initial sync should succeed");
        assert!(!first.written.is_empty(), "expected migrations to be written");
        assert!(first.skipped.is_empty(), "first sync should not skip");

        for written in &first.written {
            let file_name = written
                .file_name()
                .and_then(|n| n.to_str())
                .expect("file name should be valid utf-8");
            assert!(
                file_name.contains("__underlay_auth__"),
                "unexpected synced filename: {file_name}"
            );
        }

        let second = sync_migrations(&path, false).expect("second sync should succeed");
        assert!(second.written.is_empty(), "second sync should not write");
        assert_eq!(
            second.skipped.len(),
            first.written.len(),
            "all synced files should be skipped when unchanged"
        );
    }

    #[test]
    fn sync_migrations_detects_content_mismatch() {
        let path = temp_dir("underlay_sync_mismatch");
        let first = sync_migrations(&path, false).expect("initial sync should succeed");

        let changed = first
            .written
            .first()
            .expect("at least one migration should be written");
        std::fs::write(changed, "-- modified by test\n").expect("file should be writable");

        let err = sync_migrations(&path, false).expect_err("mismatch should fail");
        match err {
            SyncMigrationsError::ContentMismatch { path: mismatch_path } => {
                assert_eq!(mismatch_path, *changed)
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
