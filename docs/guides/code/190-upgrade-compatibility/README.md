# 190 - Upgrade Compatibility Support Files

Use these support files when an Underlay batch changes behavior, public APIs, configuration, migrations, or recommended integration patterns.

Files:

1. `feature-upgrade-note-template.md` - for feature guides, roadmap closeout, or subsystem-specific compatibility notes
2. `release-log-upgrade-block-template.md` - for delivery logs that need a compact consumer-facing upgrade summary

Rule:

- If the batch changes how a consuming app upgrades, use one of these templates instead of writing an ad hoc note.

Next task:

- Keep these templates aligned with `docs/guides/190-upgrade-compatibility.md` whenever the compatibility contract changes.
