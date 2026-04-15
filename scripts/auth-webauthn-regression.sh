#!/usr/bin/env bash
set -euo pipefail

SONGSPROUT_BLOOM_DIR="/Users/tom/Dev/projects/songsprout/bloom"
ACOWTANCY_DAIRY_DIR="/Users/tom/Dev/projects/acowtancy/dairy"

printf 'Auth WebAuthn Regression\n'
printf 'Date: %s\n\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

printf '## Songsprout Bloom (server auth actions)\n'
(
  cd "$SONGSPROUT_BLOOM_DIR"
  bun x vitest run \
    src/lib/server-tests/auth-login-page.server.test.ts \
    src/lib/server-tests/security-page.server.test.ts
)

printf '\n## Acowtancy Dairy (browser-path auth route)\n'
(
  cd "$ACOWTANCY_DAIRY_DIR"
  bun x vitest run tests/auth-login-page.test.ts
)

printf '\nAuth WebAuthn Regression complete.\n'
