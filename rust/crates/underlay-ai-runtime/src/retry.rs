use std::collections::HashSet;
use std::time::Duration;

use async_trait::async_trait;

use crate::{AiErrorKind, AiRuntimeError, LlmClient, LlmRequest, LlmResponse, ResolvedModelRoute};

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub retriable_kinds: HashSet<AiErrorKind>,
}

impl RetryConfig {
    pub fn should_retry(&self, error: &AiRuntimeError, attempt: u32) -> bool {
        attempt < self.max_attempts && self.retriable_kinds.contains(&error.kind)
    }

    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let exponent = attempt.saturating_sub(1).min(31);
        let multiplier = 1_u32 << exponent;
        let delay = self.base_delay.saturating_mul(multiplier);
        delay.min(self.max_delay)
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(3),
            retriable_kinds: default_retriable_error_kinds(),
        }
    }
}

pub fn default_retriable_error_kinds() -> HashSet<AiErrorKind> {
    HashSet::from([
        AiErrorKind::RateLimit,
        AiErrorKind::Timeout,
        AiErrorKind::Provider,
        AiErrorKind::Unknown,
    ])
}

#[derive(Debug, Clone)]
pub struct RetryMiddleware<C> {
    inner: C,
    config: RetryConfig,
}

impl<C> RetryMiddleware<C> {
    pub fn new(inner: C, config: RetryConfig) -> Self {
        Self { inner, config }
    }

    pub fn config(&self) -> &RetryConfig {
        &self.config
    }
}

#[async_trait]
impl<C> LlmClient for RetryMiddleware<C>
where
    C: LlmClient,
{
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        let mut attempt = 1;

        loop {
            match self.inner.generate_structured(route, request).await {
                Ok(response) => return Ok(response),
                Err(error) if self.config.should_retry(&error, attempt) => {
                    let delay = self.config.delay_for_attempt(attempt);
                    if !delay.is_zero() {
                        tokio::time::sleep(delay).await;
                    }
                    attempt += 1;
                }
                Err(error) => return Err(error),
            }
        }
    }
}
