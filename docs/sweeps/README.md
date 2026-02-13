# Underlay Sweeps

Sweeps are prescripted cross-repository checks used to audit Underlay-based projects for consistency, security, and operational guardrails.

Unlike implementation guides, sweeps are runbooks for reviewing an existing codebase.

## When to use sweeps

- Before a release or major deploy.
- During onboarding to a new Underlay consumer project.
- As part of periodic platform health checks (for example, monthly or quarterly).
- After incidents to validate that key guardrails still hold.

## Sweep catalogue

| Sweep | Focus | Audience |
|-------|-------|----------|
| [001-security-sweep.md](./001-security-sweep.md) | Comprehensive security review across API, client, and frontends | Platform/security reviewers |
| [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md) | Detect and correct reimplementation of existing Underlay UI/pattern functionality | Frontend/platform reviewers |
| [003-frontend-consistency-sweep.md](./003-frontend-consistency-sweep.md) | Cross-site consistency checks for frontend architecture, API usage, state, and UX patterns | Frontend/platform reviewers |

## How to run a sweep

1. Choose the sweep document.
2. Set project-specific repo paths (API, admin, web, client).
3. Run each command/check in order.
4. Record findings using the report template in that sweep.
5. Track remediation work in your project roadmap or issue tracker.

## Authoring new sweeps

When adding a new sweep:

1. Add a new numbered file in this folder (for example `002-accessibility-sweep.md`).
2. Keep commands runnable and copy/paste ready.
3. Distinguish "pass criteria" from "manual review required" cases.
4. Link to relevant Underlay guides/patterns for remediation.
5. Add the new sweep to the catalogue table above.
