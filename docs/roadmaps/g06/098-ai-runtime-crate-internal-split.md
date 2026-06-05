# g06.098 - AI Runtime Crate Internal Split

## Why

`g06.097` found that `underlay-ai-runtime/src/lib.rs` mixes public AI runtime
models, error helpers, provider registry behavior, route candidate selection,
OpenAI-compatible transport wiring, metadata/status helpers, and the stub
client in one production file.

Retry, circuit-breaker, and route-chain behavior are already modular. The next
split should complete the production crate front door without changing the
contract-listed API.

## Goal

Split the AI runtime crate front door into focused internal modules while
preserving all crate-root public exports and runtime behavior.

## Scope

In scope:

- keep `lib.rs` as the small crate front door
- move `AiErrorKind`, `AiRuntimeError`, and helper methods into an error module
- move request, response, token usage, route, and capability types into a types
  module
- move `LlmClient` into a focused client module
- move `ProviderRegistry` into a focused registry module
- move `select_route_candidates()` into a focused routing module
- move `OpenAiCompatibleClient`, wire structs, HTTP status mapping, and
  provider metadata filtering into a focused OpenAI-compatible client module
- move `StubLlmClient` into a focused stub module
- preserve retry, circuit-breaker, and route-chain re-exports
- preserve private test access to status mapping and metadata filtering helpers
- preserve existing tests

Out of scope:

- changing AI runtime public APIs
- changing route selection behavior
- changing OpenAI-compatible transport semantics
- changing retry, circuit-breaker, or route-chain behavior
- changing consumer apps

## Acceptance Criteria

- `lib.rs` becomes a small crate front door
- responsibility groups live in focused modules
- contract-listed crate-root exports remain stable
- AI runtime tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, method names, or runtime
semantics must change, stop and re-enter planning.

## Current State

`g06.098` is complete.

Artifact:

- [098 artifact](./098-ai-runtime-crate-internal-split-artifact.md)

## Next Task

Execute `g06.099`: jobs Postgres auth cleanup modularity audit.
