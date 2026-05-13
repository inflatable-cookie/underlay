# Feature Upgrade Note Template

Use this template in guides, roadmap closeout sections, or dedicated compatibility notes when an Underlay feature changes how consuming apps upgrade.

```md
## Upgrade Note: <feature or subsystem>

### Impact Class

`additive` | `deprecation` | `breaking`

### Who Needs to Act

- Apps that use `<crate/package/component/pattern>`
- Apps that currently rely on `<old behavior or config>`

### Required App Actions

1. Update dependencies:
   - `<bun install / cargo update / lockfile step>`
2. Apply code changes:
   - `<imports / config / API usage / component usage>`
3. Apply data or migration changes:
   - `<SQL migration / copied file / backfill / manifest update>`
4. Update local docs or manifests:
   - `<config/env-manifest.txt / config docs / internal runbook>`

### Deprecation or Cutover Timeline

- Ship date: `<YYYY-MM-DD>`
- Warning window ends: `<YYYY-MM-DD>` or `n/a`
- Removal date: `<YYYY-MM-DD>` or `n/a`

### Validation Commands

```bash
<consumer validation commands>
```

### Breakage Signals

- `<symptom>` -> `<likely cause>`
- `<symptom>` -> `<likely cause>`

### Docs Updated in the Same Batch

- `<guide>`
- `<pattern>`
- `<roadmap or log>`

### Rollback Notes

- `<how to revert or pause adoption safely>`
```

Next task:

- Replace placeholders with exact package names, concrete dates, and the commands the consuming app should actually run.
