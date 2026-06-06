//! Cron-based scheduler for recurring Postgres-backed jobs.

use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use tracing::{debug, error, info, instrument, warn};

use crate::postgres::{JobRepository, Result};
use crate::postgres_scheduled::ScheduledTaskRepository;
use underlay_jobs::{JobConfig, ScheduledTask, ScheduledTaskDefinition};

/// Scheduler for recurring tasks.
///
/// Checks scheduled tasks and creates jobs when they are due.
pub struct Scheduler {
    job_repo: JobRepository,
    task_repo: ScheduledTaskRepository,
}

impl Scheduler {
    /// Create a new scheduler.
    pub fn new(job_repo: JobRepository, task_repo: ScheduledTaskRepository) -> Self {
        Self {
            job_repo,
            task_repo,
        }
    }

    /// Register scheduled task definitions.
    ///
    /// This upserts the definitions into the database and disables any tasks
    /// that are no longer in the provided list.
    #[instrument(skip(self, tasks))]
    pub async fn register_tasks(&self, tasks: &[ScheduledTaskDefinition]) -> Result<()> {
        info!(count = tasks.len(), "Registering scheduled tasks");

        for task in tasks {
            if Schedule::from_str(task.schedule).is_err() {
                error!(
                    task_name = task.name,
                    schedule = task.schedule,
                    "Invalid cron expression, skipping task"
                );
                continue;
            }

            self.task_repo.upsert(task).await?;
            debug!(task_name = task.name, "Registered scheduled task");
        }

        let active_names: Vec<&str> = tasks.iter().map(|task| task.name).collect();
        let disabled = self.task_repo.disable_stale(&active_names).await?;
        if disabled > 0 {
            info!(count = disabled, "Disabled stale scheduled tasks");
        }

        Ok(())
    }

    /// Run one scheduling tick.
    #[instrument(skip(self))]
    pub async fn tick(&self) -> Result<()> {
        let tasks = self.task_repo.get_enabled().await?;
        let now = Utc::now();

        for task in tasks {
            if let Err(error) = self.check_task(&task, now).await {
                error!(
                    task_name = %task.name,
                    error = %error,
                    "Failed to check scheduled task"
                );
            }
        }

        Ok(())
    }

    async fn check_task(&self, task: &ScheduledTask, now: DateTime<Utc>) -> Result<()> {
        let schedule = match Schedule::from_str(&task.schedule) {
            Ok(schedule) => schedule,
            Err(error) => {
                warn!(
                    task_name = %task.name,
                    schedule = %task.schedule,
                    error = %error,
                    "Invalid cron expression"
                );
                return Ok(());
            }
        };

        let should_schedule = match task.last_scheduled_at {
            None => true,
            Some(last_scheduled_at) => schedule
                .after(&last_scheduled_at)
                .next()
                .map(|next| next <= now)
                .unwrap_or(false),
        };

        if !should_schedule {
            return Ok(());
        }

        if !task.allow_overlap && self.task_repo.is_running(&task.job_type).await? {
            debug!(
                task_name = %task.name,
                "Skipping schedule - previous job still running"
            );
            return Ok(());
        }

        let config = JobConfig::default()
            .with_max_attempts(task.max_attempts as u32)
            .with_optional_timeout(task.timeout_seconds.map(|seconds| seconds as u32))
            .with_allow_overlap(task.allow_overlap)
            .with_priority(task.priority);

        let job_id = self
            .job_repo
            .create(&task.job_type, task.payload.clone(), &config)
            .await?;

        self.task_repo.mark_scheduled(task.id).await?;

        info!(
            task_name = %task.name,
            job_id = %job_id,
            job_type = %task.job_type,
            "Created scheduled job"
        );

        Ok(())
    }
}
