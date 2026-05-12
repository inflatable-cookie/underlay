# g05.005 — System Index Page Template Proof

## Why

Every admin app has a system index page with the same broad shape:

- one header
- nav-card grid
- links into jobs, errors, audit, scheduled tasks, and related operator lanes

That is now repeated enough to deserve a retained template instead of six
hand-built route shells.

## Goal

Prove a shared `SystemIndexPage` shell in `underlay-reference`, then migrate the
other admin apps onto it.

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
- optional subtitle
- grouped nav-card sections
- optional top-level metrics or helper copy

Keep local:

- app-specific destination list
- app-specific card labels and descriptions

## Execution posture

1. Normalize the common system-index shape from the six current routes.
2. Prove the template in `underlay-reference`.
3. Roll it across the remaining five admin apps.
4. Add contract and usage guidance.

## Consumer Upgrade Impact

Expected.

This introduces a retained shared shell for admin system index pages.

## Next Task

Audit the six current system index pages and freeze the minimum retained shell
before implementation.
