# AI Runtime Extraction From Farmyard

Date: 2026-02-10

## Goal

Extract reusable AI runtime routing primitives from Farmyard into Underlay so other consuming apps can adopt the same architecture.

## What was extracted

- New Underlay crate: `underlay-ai-runtime`
  - Provider-agnostic contracts (`LlmClient`, request/response/error types)
  - Provider registry (`ProviderRegistry`)
  - Route candidate capability selection helper (`select_route_candidates`)
  - OpenAI-compatible transport client (`OpenAiCompatibleClient`)
  - Safe provider metadata passthrough (allowlisted keys)
  - Metadata-only provider error messages by default

## Farmyard integration changes

- Farmyard now consumes `underlay-ai-runtime` from `farmyard-infra`.
- Farmyard retains app-specific mapping and policy in `farmyard-infra::ai_runtime`:
  - Action key -> alias defaults
  - Route-chain construction from app config
  - Config adapter function for client construction

## Documentation updates in Underlay

- Added guide: `docs/guides/176-ai-runtime-routing.md`
- Added pattern index entry in `docs/patterns/000-index.md`
- Added project sync checklist entries in `docs/guides/200-project-sync.md`
- Updated package map and crate inventory docs

## What remains app-specific by design

- Routing DB schema (provider/alias/binding tables)
- Admin diagnostics API shapes and endpoints
- Rollout flags and per-app action-prefix strategy
- Dead-letter semantics and queue backpressure thresholds

These are intentionally app-layer concerns and should compose on top of Underlay runtime primitives.
