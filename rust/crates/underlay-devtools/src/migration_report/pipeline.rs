use std::path::Path;

use underlay_migration_core::PipelineRunReport;

use super::json::parse_json;
use super::MigrationReportError;

pub fn load_pipeline_run_report(path: &Path) -> Result<PipelineRunReport, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    parse_json::<PipelineRunReport>(&bytes)
}

pub fn load_pipeline_run_report_from_path(
    input: &Path,
) -> Result<PipelineRunReport, MigrationReportError> {
    if input.is_file() {
        return load_pipeline_run_report(input);
    }

    if input.is_dir() {
        let mut files = std::fs::read_dir(input)
            .map_err(MigrationReportError::Io)?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();
        files.sort();
        for file in files {
            if let Ok(report) = load_pipeline_run_report(&file) {
                return Ok(report);
            }
        }
        return Err(MigrationReportError::InvalidInput(format!(
            "no pipeline run report JSON found in {}",
            input.display()
        )));
    }

    Err(MigrationReportError::InvalidInput(format!(
        "input path does not exist: {}",
        input.display()
    )))
}
