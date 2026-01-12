# Backlog: CLI Scaffolding

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 10-15 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Creating CRUD endpoints is repetitive. New developers spend time on boilerplate instead of business logic.

---

## Proposed Solution

CLI tool to generate boilerplate code from templates.

### Potential Commands

```bash
# Generate CRUD endpoint
underlay generate crud users

# Generate database migration
underlay generate migration add_users_table

# Generate Svelte component
underlay generate component DataCard

# Generate API client command
underlay generate command users

# Generate test file
underlay generate test users
```

### Example Output

```bash
$ underlay generate crud articles

Created:
  src/handlers/articles.rs    (CRUD handlers)
  src/models/article.rs       (Domain model)
  src/db/articles.rs          (Database queries)
  migrations/xxx_articles.sql (Database migration)
  tests/articles.rs           (Integration tests)

Next steps:
  1. Review generated code
  2. Run migration: sqlx migrate run
  3. Add routes to main.rs
```

---

## Dependencies

- Stable patterns (no changes in 3+ months)
- Template engine (Tera, Handlebars)
- CLI framework (clap)

---

## When to Build

- Creating 10+ similar endpoints feels painful
- Patterns are very stable
- Onboarding new developers takes too long
- Team requests it

---

## Why Deferred

Wait until patterns are very stable. Hand-coding ensures developers understand the code. Premature scaffolding can:
- Encourage cargo-culting
- Generate code developers don't understand
- Lock in patterns that should evolve
- Create maintenance burden for templates

---

## Success Criteria

- [ ] Generate CRUD handlers with standard patterns
- [ ] Generate database migrations
- [ ] Generate Svelte components
- [ ] Templates are customizable per project
- [ ] Generated code passes linting/formatting
- [ ] Documentation for template customization

---

## Risks & Considerations

- Templates need maintenance as patterns evolve
- Different projects have different patterns
- May over-generate code
- May encourage bad patterns if not careful
- Learning curve for template customization

---

**Created**: 2026-01-12
