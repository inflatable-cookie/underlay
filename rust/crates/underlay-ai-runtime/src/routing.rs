use std::collections::HashSet;

use crate::{ModelCapability, ResolvedModelRouteCandidate};

pub fn select_route_candidates(
    mut candidates: Vec<ResolvedModelRouteCandidate>,
    required_capabilities: &HashSet<ModelCapability>,
) -> Vec<ResolvedModelRouteCandidate> {
    candidates.retain(|candidate| {
        required_capabilities
            .iter()
            .all(|capability| candidate.capabilities.contains(capability))
    });

    candidates.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.route.provider_name.cmp(&b.route.provider_name))
            .then_with(|| a.route.model_name.cmp(&b.route.model_name))
    });

    candidates
}
