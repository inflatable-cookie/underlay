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

Audit the six dashboard routes and classify the minimum shared shell versus
app-local metric content.
