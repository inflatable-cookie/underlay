# 011 - Underlay Template Skill

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

Developers need a quick way to look up template documentation without hunting
through files. A single skill with a small front door delegates to the full
template docs.

## Goals

- create `~/.agents/skills/underlay-template/SKILL.md`
- small front door with commands that open relevant docs
- no code generation — just documentation lookup

## Skill Commands

```
/underlay-template list      → docs/usage/templates/entity-list-page.md
/underlay-template detail    → docs/usage/templates/entity-detail-page.md
/underlay-template form      → docs/usage/templates/entity-form-page.md
/underlay-template section   → docs/usage/templates/template-sections-guide.md
/underlay-template overview  → docs/usage/templates/000-template-system-overview.md
```

## Exit Criteria

- skill file exists at `~/.agents/skills/underlay-template/SKILL.md`
- all commands resolve to correct docs
- skill is documented in Underlay's developer onboarding

## Next Task

Execute `g03.012`: plan the consumer rollout to all 6 apps.
