# g06.098 Artifact - AI Runtime Crate Internal Split

## Summary

`underlay-ai-runtime/src/lib.rs` is now a small crate front door with stable
crate-root exports. The former mixed production file was split into focused
modules.

New module shape:

- `lib.rs`: front door, module declarations, public re-exports, and test module
- `error.rs`: `AiErrorKind`, `AiRuntimeError`, and helper methods
- `types.rs`: request, response, token usage, route, and capability public
  types
- `client.rs`: `LlmClient`
- `registry.rs`: `ProviderRegistry`
- `routing.rs`: `select_route_candidates()`
- `openai.rs`: `OpenAiCompatibleClient`, wire structs, provider metadata
  filtering, and HTTP status mapping
- `stub.rs`: `StubLlmClient`

Retry, circuit-breaker, and route-chain modules remain unchanged and are still
re-exported from the crate front door.

## Public API Impact

None expected.

The contract-listed crate-root exports, retry/circuit/route-chain re-exports,
route selection behavior, OpenAI-compatible client behavior, private
test-visible mapping helpers, and stub client behavior were preserved.

## Validation

- `cargo test -p underlay-ai-runtime --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 39 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The AI runtime crate front door no longer appears in the god-file report. The
next largest Rust production warning is
`rust/crates/underlay-jobs-postgres/src/tasks/auth_cleanup.rs`.

## Next Target Evidence

Queue `g06.099` as a jobs Postgres auth cleanup modularity audit before
splitting `auth_cleanup.rs`. It is a shared job task surface, so the next batch
should classify task configuration, repository queries, cleanup behavior,
reporting, and tests before moving code.
