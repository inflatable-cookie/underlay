# Release Protocol

> **Interim placement.** A dedicated `effigy-release` skill is planned. Until
> it ships, this reference holds the strict release rules.

## Top rule

**Do not run release commands without explicit human instruction.** Releases
have public side effects — tags, GitHub releases, distribution manifest
updates — and a wrong release is hard to recover from cleanly.

See also `footguns.md` rules 1–4.

## Read-only release commands (safe to run unprompted)

Use these for inspection, planning, and reporting:

| Command | What it does |
|---------|--------------|
| `effigy release simulate` | Dry-run the entire release flow |
| `effigy release status --check-gates` | Show current gate states |
| `effigy release prepare --plan` | Preview prepare step |
| `effigy release execute --plan` | Preview execute step |
| `effigy release gates` | List release gates |
| `effigy changelog extract --version X.Y.Z` | Extract changelog section |

## Mutating release commands (require explicit human ask)

Never run these unprompted:

| Command | Side effect |
|---------|-------------|
| `effigy release prepare --yes` | Writes prepare artifacts |
| `effigy release execute --yes` | Commits prepared files and pushes the annotated tag |
| `gh release create vX.Y.Z …` | Publishes the GitHub Release for an existing tag |
| `effigy release verify-install --tag vX.Y.Z` | Effigy binary network-side verification; never use for Underlay |

`release execute` does **not** create the GitHub Release. Underlay publishes
that provider surface as a separate operator step after the tag exists.

## Canonical sequence

When a human explicitly asks for a release:

1. `effigy release simulate`
   - confirm the mutation plan updates both `Cargo.toml` and `package.json` to
     the selected version
2. `effigy release status --check-gates`
3. `effigy release prepare --plan`
4. `effigy release prepare --yes --check-gates`
5. `effigy release execute --plan`
6. `effigy release execute --yes`
7. Publish the GitHub Release for the new tag (operator-owned), for example
   `gh release create vX.Y.Z --generate-notes` or an equivalent provider
   command. Do not treat execute success as proof the public release exists.
8. Run Underlay's tagged consumer smoke. Do not run Effigy's fixed binary
   verifier.
9. `effigy changelog extract CHANGELOG.md --version X.Y.Z`

If any step fails, **stop**. Surface the failure to the human. Do not retry
with bypass flags.

`release verify-install` installs the `effigy` Cargo package and runs Effigy
CLI checks. Invoking it from Underlay is a routing error, not evidence that an
Underlay tag is broken.

## Failed release recovery

If `release execute` fails partway through:

1. **Do not re-tag.** The original tag may already be on the remote.
2. Identify the underlying cause from gate output or error envelope.
3. Add a `Fixed` entry to `CHANGELOG.md` under `[Unreleased]` describing the
   problem.
4. Bump the next PATCH version.
5. Run the standard release flow for the new version.

The broken release stays in history; consumers who pulled it learn the fix
landed in the next PATCH.

## CI workflow files

`.github/workflows/` files gate the release pipeline. **Never modify them
without explicit human approval.** A silent edit can:

- Skip a required gate.
- Leak secrets.
- Break artifact uploads.
- Corrupt the distribution manifest.

If a workflow change is needed, propose it to the human and let them apply.

## Full spec

- `docs/guides/049-ci-binary-distribution-and-release-protocol.md` — CI
  policy, distribution rules, install protocol.
- `docs/guides/051-release-orchestration.md` — release command sequence,
  manifest format, gate definitions.
