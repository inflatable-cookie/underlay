use std::collections::HashSet;

use super::super::{select_route_candidates, ModelCapability};
use super::support::route;

#[test]
fn route_selection_is_deterministic_for_ties() {
    let selected = select_route_candidates(
        vec![
            route("router-b", "model-z", 10),
            route("router-a", "model-z", 10),
            route("router-a", "model-a", 10),
        ],
        &HashSet::from([ModelCapability::StructuredJson]),
    );

    assert_eq!(selected[0].route.provider_name, "router-a");
    assert_eq!(selected[0].route.model_name, "model-a");
    assert_eq!(selected[1].route.provider_name, "router-a");
    assert_eq!(selected[1].route.model_name, "model-z");
    assert_eq!(selected[2].route.provider_name, "router-b");
    assert_eq!(selected[2].route.model_name, "model-z");
}

#[test]
fn route_selection_filters_by_capability() {
    let mut with_tools = route("router-a", "model-a", 1);
    with_tools.capabilities.insert(ModelCapability::ToolCalling);

    let selected = select_route_candidates(
        vec![with_tools, route("router-b", "model-b", 1)],
        &HashSet::from([
            ModelCapability::StructuredJson,
            ModelCapability::ToolCalling,
        ]),
    );

    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].route.provider_name, "router-a");
}
