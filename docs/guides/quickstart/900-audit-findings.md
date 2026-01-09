# Documentation Audit Report

## Overview

This document tracks quickstart issues found by auditing the guide against:

- Underlay’s actual Rust/TS types in this repo
- Acowtancy / Songsprout architectural patterns

## Current Status

Most issues originally identified have now been addressed. Remaining gaps are primarily about completeness of *application-specific* implementation details (which is expected).

## Key Fixes Applied

- ✅ **Response envelopes aligned** with `underlay_core::{ListResponse, SingleResponse}` and TS `ListResponse/SingleResponse`.
- ✅ **Error envelopes aligned** with Underlay’s canonical `{ error: { code, message, fieldErrors? } }` shape.
- ✅ **API versioning clarified**: stable `/v1/...` URLs with optional `X-Api-Version` header (Acowtancy-style).
- ✅ **Handlers docs fixed**: removed Acowtancy-specific crate references and `/api/v1` paths.
- ✅ **Database docs fixed**: migrations moved to `crates/db/migrations`, `pgcrypto` extension called out, removed invalid `sqlx::migrate!(migrations_path)` usage.
- ✅ **Broken code-example references fixed**: created missing `code/140-local-development/` and `code/160-troubleshooting/` assets.
- ✅ **AGENTS.md contradiction fixed**: clarified root-level “repo plumbing” files are allowed.

## Files Status

| Document | Notes |
|----------|-------|
| `docs/guides/quickstart/030-underlay-integration.md` | Updated TS envelope example to match Underlay |
| `docs/guides/quickstart/050-database.md` | Updated migrations layout + SQL fixes |
| `docs/guides/quickstart/060-authentication.md` | Removed `farmyard_core` references; removed stray `todo!()` |
| `docs/guides/quickstart/070-api-handlers.md` | Updated to `/v1`, Underlay envelopes/errors |
| `docs/guides/quickstart/080-typescript-client.md` | Updated to use Underlay TS client + correct envelopes |
| `docs/guides/quickstart/140-local-development.md` | Added concrete code example references |
| `docs/guides/quickstart/160-troubleshooting.md` | Added concrete code example references |

## Remaining Known Limitations

These are expected (and OK for a quickstart), but worth noting:

- JWT validation in the Rust quickstart is intentionally not fully implemented (Underlay does not ship a JWT implementation).
- The guide provides patterns and scaffolding; domain-specific repository implementations, DTOs, and endpoints are examples.

## Canonical Reference

When in doubt, use Acowtancy as the canonical implementation:
- `/Users/betterthanclay/Dev/apps/acowtancy/farmyard/crates/` - Rust backend
- `/Users/betterthanclay/Dev/apps/acowtancy/cream/src/` - User frontend
- `/Users/betterthanclay/Dev/apps/acowtancy/dairy/src/` - Admin frontend
