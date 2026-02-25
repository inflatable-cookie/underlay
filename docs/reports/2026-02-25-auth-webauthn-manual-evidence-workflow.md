# Auth WebAuthn Manual Evidence Workflow (2026-02-25)

## Purpose

Provide a repeatable way to generate a single manual-evidence dossier for non-OAuth WebAuthn verification across Songsprout and Dairy.

## Script

- `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-manual-dossier.sh`

What it does:
1. Runs readiness gate (`auth-live-e2e-readiness.sh`) with current environment.
2. Runs automated passkey/WebAuthn regression (`auth-webauthn-regression.sh`).
3. Writes a timestamped report file with:
   - automated baseline output
   - manual checklist
   - result table for screenshot/evidence entries

## Recommended command in this environment

- `SKIP_GOOGLE_OAUTH_CHECKS=1 /Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-manual-dossier.sh`

## Latest generated dossier

- `/Users/betterthanclay/Dev/projects/underlay/docs/reports/2026-02-25-105622-auth-webauthn-manual-evidence-dossier.md`

## Usage notes

- Start Songsprout API first (`bun run api` in `songsprout/nursery`) so readiness is green.
- OAuth checks remain intentionally deferred when credentials are unavailable.
