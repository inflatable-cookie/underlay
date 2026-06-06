use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    AiErrorKind, AiRuntimeError, LlmRequest, LlmResponse, ProviderRegistry, ResolvedModelRoute,
};

#[derive(Debug, Clone)]
pub struct RouteChainConfig {
    fallback_error_kinds: HashSet<AiErrorKind>,
}

impl RouteChainConfig {
    pub fn fallback_error_kinds(&self) -> &HashSet<AiErrorKind> {
        &self.fallback_error_kinds
    }

    pub fn with_fallback_error_kinds(mut self, fallback_error_kinds: HashSet<AiErrorKind>) -> Self {
        self.fallback_error_kinds = fallback_error_kinds;
        self
    }

    pub fn should_fallback(&self, error: &AiRuntimeError) -> bool {
        self.fallback_error_kinds().contains(&error.kind)
    }
}

impl Default for RouteChainConfig {
    fn default() -> Self {
        Self {
            fallback_error_kinds: default_fallback_error_kinds(),
        }
    }
}

pub fn default_fallback_error_kinds() -> HashSet<AiErrorKind> {
    HashSet::from([
        AiErrorKind::Auth,
        AiErrorKind::RateLimit,
        AiErrorKind::Timeout,
        AiErrorKind::Provider,
        AiErrorKind::Unknown,
        AiErrorKind::CircuitOpen,
    ])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteChainAttempt {
    pub route: ResolvedModelRoute,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AiRuntimeError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteChainResult {
    pub response: LlmResponse,
    pub route: ResolvedModelRoute,
    pub route_index: usize,
    pub attempts: Vec<RouteChainAttempt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteChainFailure {
    pub error: AiRuntimeError,
    pub attempts: Vec<RouteChainAttempt>,
}

pub struct RouteChainExecutor {
    registry: ProviderRegistry,
    config: RouteChainConfig,
}

impl RouteChainExecutor {
    pub fn new(registry: ProviderRegistry) -> Self {
        Self {
            registry,
            config: RouteChainConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RouteChainConfig) -> Self {
        self.config = config;
        self
    }

    pub async fn execute_with_fallback(
        &self,
        chain: &[ResolvedModelRoute],
        request: &LlmRequest,
    ) -> Result<RouteChainResult, RouteChainFailure> {
        if chain.is_empty() {
            return Err(RouteChainFailure {
                error: AiRuntimeError::new(
                    AiErrorKind::Validation,
                    "Route chain must include at least one route.",
                ),
                attempts: Vec::new(),
            });
        }

        let mut attempts = Vec::with_capacity(chain.len());
        let mut last_error = None;

        for (route_index, route) in chain.iter().cloned().enumerate() {
            let Some(client) = self.registry.get(&route.provider_name) else {
                let error = AiRuntimeError::new(
                    AiErrorKind::Provider,
                    format!(
                        "No LLM client registered for provider `{}`.",
                        route.provider_name
                    ),
                );
                attempts.push(RouteChainAttempt {
                    route,
                    error: Some(error.clone()),
                });

                if route_index + 1 < chain.len() && self.config.should_fallback(&error) {
                    last_error = Some(error);
                    continue;
                }

                return Err(RouteChainFailure { error, attempts });
            };

            match client.generate_structured(&route, request).await {
                Ok(response) => {
                    attempts.push(RouteChainAttempt {
                        route: route.clone(),
                        error: None,
                    });
                    return Ok(RouteChainResult {
                        response,
                        route,
                        route_index,
                        attempts,
                    });
                }
                Err(error) => {
                    attempts.push(RouteChainAttempt {
                        route,
                        error: Some(error.clone()),
                    });

                    if route_index + 1 < chain.len() && self.config.should_fallback(&error) {
                        last_error = Some(error);
                        continue;
                    }

                    return Err(RouteChainFailure { error, attempts });
                }
            }
        }

        Err(RouteChainFailure {
            error: last_error.unwrap_or_else(|| {
                AiRuntimeError::new(
                    AiErrorKind::Unknown,
                    "Route chain exhausted without a terminal success.",
                )
            }),
            attempts,
        })
    }
}
