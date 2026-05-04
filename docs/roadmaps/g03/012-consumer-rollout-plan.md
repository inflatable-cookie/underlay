# 012 - Consumer Rollout Plan

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

Once templates are proven in acme-admin and Dairy, they need to roll out to the
full consumer family. This roadmap plans that rollout.

## Consumer Family

1. `acowtancy/dairy` — admin (complex, already partially validated in g03.010)
2. `underlay-reference/acme-admin` — reference (proven in g03.006–009)
3. `contact-patch/cp-admin` — admin
4. `compli-me` — admin
5. `songsprout` — mixed
6. `loophole/composer` — mixed

## Rollout Order

1. acme-admin (reference) — already done as proof
2. Dairy — complex validation in g03.010
3. cp-admin — similar to acme-admin, straightforward
4. compli-me — first non-Acowtancy consumer
5. songsprout — mixed consumer
6. loophole/composer — mixed consumer

## Goals

- plan the rollout sequence
- define per-app scope (which pages get templated first)
- define rollback criteria if a template doesn't fit an app's shape

## Exit Criteria

- rollout plan documented in `docs/usage/templates/consumer-rollout.md`
- per-app scope and priority defined
- rollback criteria defined

## Next Task

Execute `g03.013`: build `EntityForm` and `EntityFormPage` — the form template
family (create/edit/modal).
