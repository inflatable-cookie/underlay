# Underlay Foundation Vision

## Purpose

Underlay exists to give projects a reusable full-stack foundation without forcing them into one product domain.

It should make the strong parts of the current portfolio easy to reuse:

- Rust backend primitives
- typed TypeScript client contracts
- shared Svelte UI components and patterns
- auth, observability, storage, and operations foundations
- repeatable guidance for spinning up and evolving new projects

## Long-Term Outcome

Underlay should make shared full-stack foundations easy to adopt across real
projects without turning those projects into one rigid framework or hiding the
boundary between reusable infrastructure and product-specific code.

## Core Platform Behaviors

- stable primitives instead of app-specific abstractions
- composable boundaries that consuming apps can adopt incrementally
- strong contract clarity across Rust, TypeScript, and Svelte layers
- practical extraction from real projects rather than speculative framework design
- documentation that helps both humans and fresh agents use the library correctly

## Strategic Constraints

Underlay is not:

- a product with its own domain workflows
- a place for app-specific roadmap or business logic drift
- a managed platform that hides every infrastructure decision
- a replacement for consuming apps owning their own domain model and product intent

## Longer-Term Focus

Over time, Underlay should:

- keep extracting the minimum durable common layer from active projects
- provide sharper migration, auth, and operations primitives where those patterns repeat
- make new project bootstrap materially faster without encouraging low-rigor scaffolding
- remain small enough that consuming teams can understand and challenge its boundaries

## Next Task

Open the current roadmap of the active generation `g09` (contract fidelity and
fleet convergence) via [`docs/roadmaps/README.md`](../roadmaps/README.md), keeping
the foundation vision's execution handoff current.
