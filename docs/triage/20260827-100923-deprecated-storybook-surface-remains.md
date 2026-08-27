# Deprecated Storybook Surface Remains

Status: open
Captured: 2026-08-27

## Observation

The Storybook backlog was removed as obsolete, but `effigy.toml` still exposes
`storybook` and `storybook:build`, while the root README and active UI guides
still direct maintainers to the local Storybook catalog.

## Impact

The runnable task inventory and live documentation advertise a deprecated UI
discovery surface after the cleanup decision.

## Disposition

Keep open. Route removal of the task selectors, dependencies/configuration,
stories, and active-guide references through one bounded cleanup handoff. Do
not mix it into the g09.047 runtime/access promotion.
