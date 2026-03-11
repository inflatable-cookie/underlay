# Release Log Upgrade Block Template

Use this compact block inside `docs/logs/YYYY-MM/*.md` when the batch changes consumer-visible behavior.

```md
## Consumer Upgrade Notes

- Impact class: `<additive|deprecation|breaking>`
- Affected consumers: `<apps or subsystems>`
- Required actions:
  - `<dependency/config/code step>`
  - `<migration/doc step>`
- Validation:
  - `<command>`
  - `<command>`
- Deprecation/removal date: `<YYYY-MM-DD>` or `n/a`
- Reference docs:
  - `<guide>`
  - `<roadmap>`
```

Next task:

- Keep this block brief in logs and link out to the fuller feature upgrade note when the rollout has multiple steps.
