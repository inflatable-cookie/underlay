# g05.006 — Admin Dashboard Page Template Proof

## Why

The root admin dashboards are still repeated local shells across the current
consumer family. They are not entity pages, but they are repeated enough to
justify one retained dashboard shell.

## Goal

Define and prove a shared `AdminDashboardPage` shell for normal admin home
pages.

## Current inventory

Repeated in:

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `compli-me/admin`
- `contact-patch/cp-admin`
- `songsprout/greenhouse`
- `loophole/composer/composer-admin`

## Shape

Expected responsibilities:

- page header
- hero subtitle or short copy
- metric-tile zone
- nav-card zone
- optional secondary operator/status section

Keep local:

- metric definitions
- card destinations
- app-specific domain summary widgets

## Execution posture

1. Compare the six live dashboards.
2. Freeze the smallest honest retained shell.
3. Prove it in `underlay-reference`.
4. Roll it to the remaining admin apps.
5. Update the contract and usage docs.

## Consumer Upgrade Impact

Expected.

This introduces a retained dashboard shell for admin home pages.

## Next Task

Shared proof is complete.

Retained surface added:

- `AdminDashboardPage`

Proof coverage:

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `compli-me/admin`
- `contact-patch/cp-admin`
- `songsprout/greenhouse`
- `loophole/composer/composer-admin`

Shared contract/docs updated:

- `docs/contracts/110-admin-template-system.md`
- `docs/usage/templates/000-template-system-overview.md`
- `docs/usage/templates/template-api-reference.md`
- `docs/usage/templates/admin-dashboard-page.md`

The retained seam is narrow and honest:

- shared dashboard header
- shared stacked section layout
- route-owned metrics, nav cards, callouts, and app-specific widgets

Next move:

None in `g05`. The repeated shared-page lane is complete.
