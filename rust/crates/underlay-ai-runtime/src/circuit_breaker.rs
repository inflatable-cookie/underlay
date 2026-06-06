use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{AiErrorKind, AiRuntimeError, LlmClient, LlmRequest, LlmResponse, ResolvedModelRoute};

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub window_duration: Duration,
    pub reset_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            window_duration: Duration::from_secs(60),
            reset_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Default)]
struct ProviderCircuitState {
    recent_failures: VecDeque<Instant>,
    opened_at: Option<Instant>,
    half_open_in_flight: bool,
}

impl ProviderCircuitState {
    fn prune_failures(&mut self, now: Instant, window_duration: Duration) {
        while let Some(failure_at) = self.recent_failures.front().copied() {
            if now.duration_since(failure_at) <= window_duration {
                break;
            }
            self.recent_failures.pop_front();
        }
    }

    fn current_state(&self, now: Instant, config: &CircuitBreakerConfig) -> CircuitState {
        match self.opened_at {
            Some(opened_at) if now.duration_since(opened_at) < config.reset_timeout => {
                CircuitState::Open
            }
            Some(_) => CircuitState::HalfOpen,
            None => CircuitState::Closed,
        }
    }

    fn allow_request(&mut self, now: Instant, config: &CircuitBreakerConfig) -> bool {
        self.prune_failures(now, config.window_duration);
        match self.current_state(now, config) {
            CircuitState::Closed => true,
            CircuitState::Open => false,
            CircuitState::HalfOpen => {
                if self.half_open_in_flight {
                    false
                } else {
                    self.half_open_in_flight = true;
                    true
                }
            }
        }
    }

    fn record_success(&mut self) {
        self.recent_failures.clear();
        self.opened_at = None;
        self.half_open_in_flight = false;
    }

    fn record_failure(&mut self, now: Instant, config: &CircuitBreakerConfig) {
        self.prune_failures(now, config.window_duration);
        self.recent_failures.push_back(now);
        self.half_open_in_flight = false;
        if self.opened_at.is_some()
            || self.recent_failures.len() >= config.failure_threshold as usize
        {
            self.opened_at = Some(now);
        }
    }
}

#[derive(Debug)]
pub struct CircuitBreakerMiddleware<C> {
    inner: C,
    config: CircuitBreakerConfig,
    state: Mutex<HashMap<String, ProviderCircuitState>>,
}

impl<C> CircuitBreakerMiddleware<C> {
    pub fn new(inner: C, config: CircuitBreakerConfig) -> Self {
        Self {
            inner,
            config,
            state: Mutex::new(HashMap::new()),
        }
    }

    pub fn config(&self) -> &CircuitBreakerConfig {
        &self.config
    }

    pub fn provider_state(&self, provider_name: &str) -> CircuitState {
        let now = Instant::now();
        self.lock_state()
            .get(provider_name)
            .map(|state| state.current_state(now, &self.config))
            .unwrap_or(CircuitState::Closed)
    }

    fn lock_state(&self) -> MutexGuard<'_, HashMap<String, ProviderCircuitState>> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[cfg(test)]
    pub(crate) fn test_poison_state_lock(&self) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            panic!("poison circuit breaker lock");
        }));
    }
}

#[async_trait]
impl<C> LlmClient for CircuitBreakerMiddleware<C>
where
    C: LlmClient,
{
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        let now = Instant::now();
        {
            let mut state = self.lock_state();
            let provider_state = state.entry(route.provider_name.clone()).or_default();
            if !provider_state.allow_request(now, &self.config) {
                return Err(AiRuntimeError::new(
                    AiErrorKind::CircuitOpen,
                    format!(
                        "Circuit breaker is open for provider `{}`.",
                        route.provider_name
                    ),
                ));
            }
        }

        match self.inner.generate_structured(route, request).await {
            Ok(response) => {
                let mut state = self.lock_state();
                if let Some(provider_state) = state.get_mut(&route.provider_name) {
                    provider_state.record_success();
                }
                Ok(response)
            }
            Err(error) => {
                let now = Instant::now();
                let mut state = self.lock_state();
                let provider_state = state.entry(route.provider_name.clone()).or_default();
                provider_state.record_failure(now, &self.config);
                Err(error)
            }
        }
    }
}
