# 2026-06-09 - Admin section agent protocol audit

## Context

Acowtancy Dairy is now close enough to act as the live reference for
Underlay-based admin UI construction, but agent-built detail pages were still
drifting badly.

The immediate failure mode was asymmetric guidance:

- list docs and skills pushed agents toward `EntityListPage`
- detail guidance was split between older Poodle-first docs and newer
  `EntityDetailPage` template contracts
- no single skill command described how to build a whole admin route family

## Findings

- `docs/guides/180-admin-workflow-playbook.md` still mapped detail pages to
  raw `PageHeader` + `MetaBar` + `DetailSection` / `DetailItem` composition.
- `underlay-build` had separate list/detail/form commands, but no
  `admin-section` entrypoint for cohesive route-family delivery.
- `underlay-template` had component lookup commands, but no protocol command
  for agent-built admin sections.
- `docs/usage/000-overview.md` did not route template users through a cohesive
  admin-section protocol first.

## Changes

- Added `docs/usage/templates/admin-section-agent-protocol.md`.
- Linked the protocol from the template overview and API reference.
- Updated `docs/usage/000-overview.md` reading order and skill command list.
- Marked `docs/guides/180-admin-workflow-playbook.md` as legacy for UI
  composition and corrected its default visible mapping.
- Added `/underlay-build admin-section` to the Underlay build skill.
- Added `/underlay-template admin` to the Underlay template skill.

## Current State

The simple prompt path is now:

```text
Use /underlay-build admin-section.
Build the admin section for <resource family>.
Follow nearby app examples first, then Underlay template docs.
Deliver list, detail, create/edit, action menu, navigation context, counters,
and validation as one cohesive route family.
```

## Next Task

Use this protocol on the next Dairy admin-section build and record any remaining
drift as either a protocol gap or a real shared-template gap.
