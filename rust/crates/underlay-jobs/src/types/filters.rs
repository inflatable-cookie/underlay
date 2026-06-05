use super::status::JobStatus;

/// Filters for listing jobs.
#[derive(Debug, Default, Clone)]
pub struct JobFilters {
    pub status: Option<JobStatus>,
    pub job_type: Option<String>,
    pub limit: usize,
    pub offset: usize,
}

impl JobFilters {
    pub fn new() -> Self {
        Self {
            limit: 50,
            ..Default::default()
        }
    }

    pub fn with_status(mut self, status: JobStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_job_type(mut self, job_type: impl Into<String>) -> Self {
        self.job_type = Some(job_type.into());
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}
