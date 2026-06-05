use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{info, instrument};

use underlay_jobs::{Job, JobConfig, JobHandler, JobHandlerError};

/// Suspend long-inactive active accounts and revoke their active sessions.
///
/// Inactivity is determined by:
/// - latest `auth.sessions.last_used_at` for the user (status = active), else
/// - `auth.users.created_at` fallback for never-logged-in accounts.
///
/// Default policy:
/// - threshold: 1095 days (3 years)
/// - target roles: student, tester
/// - batch size: 500 users per run
/// - revoke active sessions after suspension: enabled
///
/// Recommended schedule: daily (`0 40 3 * * *`)
#[derive(Debug, Clone)]
pub struct SuspendInactiveAccountsJob {
    pool: PgPool,
    inactivity_days: i32,
    roles: Vec<String>,
    batch_limit: i64,
    revoke_sessions: bool,
}

impl SuspendInactiveAccountsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            inactivity_days: 1095,
            roles: vec!["student".to_string(), "tester".to_string()],
            batch_limit: 500,
            revoke_sessions: true,
        }
    }

    pub fn with_inactivity_days(mut self, days: i32) -> Self {
        self.inactivity_days = days.max(1);
        self
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles
            .into_iter()
            .map(|r| r.trim().to_string())
            .filter(|r| !r.is_empty())
            .collect();
        self
    }

    pub fn with_batch_limit(mut self, batch_limit: i64) -> Self {
        self.batch_limit = batch_limit.max(1);
        self
    }

    pub fn with_revoke_sessions(mut self, revoke_sessions: bool) -> Self {
        self.revoke_sessions = revoke_sessions;
        self
    }
}

#[async_trait]
impl JobHandler for SuspendInactiveAccountsJob {
    fn job_type(&self) -> &'static str {
        "suspend_inactive_accounts"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "suspend_inactive_accounts", inactivity_days = self.inactivity_days, batch_limit = self.batch_limit))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        if self.roles.is_empty() {
            info!("Skipping inactive-account suspension because no target roles were configured");
            return Ok(());
        }

        let mut tx =
            self.pool.begin().await.map_err(|e| {
                JobHandlerError::new(format!("Database transaction begin error: {e}"))
            })?;

        let suspended_user_ids = sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            WITH candidate AS (
                SELECT
                    u.id
                FROM auth.users u
                LEFT JOIN LATERAL (
                    SELECT MAX(s.last_used_at) AS last_used_at
                    FROM auth.sessions s
                    WHERE s.user_id = u.id
                      AND s.status = 'active'
                ) session_activity ON TRUE
                WHERE u.status = 'active'
                  AND u.role = ANY($1)
                  AND COALESCE(session_activity.last_used_at, u.created_at) <
                      NOW() - ($2 || ' days')::interval
                ORDER BY COALESCE(session_activity.last_used_at, u.created_at) ASC
                LIMIT $3
            )
            UPDATE auth.users u
            SET status = 'suspended',
                suspension_reason = 'inactive_account_auto_suspend',
                updated_at = NOW()
            FROM candidate c
            WHERE u.id = c.id
            RETURNING u.id
            "#,
        )
        .bind(&self.roles)
        .bind(self.inactivity_days)
        .bind(self.batch_limit)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            JobHandlerError::new(format!("Database error suspending inactive accounts: {e}"))
        })?;

        let mut revoked_sessions = 0_i64;
        if self.revoke_sessions && !suspended_user_ids.is_empty() {
            revoked_sessions = sqlx::query_scalar::<_, i64>(
                r#"
                WITH revoked AS (
                    UPDATE auth.sessions
                    SET status = 'revoked',
                        is_active = FALSE,
                        revoked_at = NOW(),
                        revocation_reason = 'inactive_account_auto_suspend'
                    WHERE user_id = ANY($1)
                      AND status = 'active'
                    RETURNING id
                )
                SELECT COUNT(*) FROM revoked
                "#,
            )
            .bind(&suspended_user_ids)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                JobHandlerError::new(format!(
                    "Database error revoking inactive-account sessions: {e}"
                ))
            })?;
        }

        tx.commit()
            .await
            .map_err(|e| JobHandlerError::new(format!("Database transaction commit error: {e}")))?;

        info!(
            suspended_accounts = suspended_user_ids.len(),
            revoked_sessions,
            inactivity_days = self.inactivity_days,
            roles = ?self.roles,
            "Processed inactive account suspension maintenance task"
        );

        Ok(())
    }
}
