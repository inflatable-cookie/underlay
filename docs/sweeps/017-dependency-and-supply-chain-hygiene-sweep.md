# 017 - Dependency and Supply Chain Hygiene Sweep

This sweep checks dependency security and maintainability across Rust and TypeScript codebases in Underlay-based projects.

## Problem this sweep targets

Common regressions:

- known vulnerable transitive dependencies left unresolved
- lockfiles drifting from manifest intent
- stale or unused dependencies increasing attack surface
- insecure install/runtime scripts introduced without review
- package source trust boundaries not documented

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Inventory dependency surfaces

```bash
rg -n "\[dependencies\]|\[dev-dependencies\]|\[workspace.dependencies\]" "$API_REPO" -g "Cargo.toml"
rg -n "\"dependencies\"|\"devDependencies\"|\"peerDependencies\"|\"optionalDependencies\"" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "package.json"
```

Pass criteria:

- dependency lists are explicit and reasonably scoped
- high-risk packages are easy to identify for targeted review

---

## Step 2 - Security advisory checks

### 2.1 Rust advisories

```bash
cd "$API_REPO" && cargo audit || true
cd "$API_REPO" && cargo deny check advisories || true
```

### 2.2 JS advisories

```bash
cd "$CLIENT_REPO" && bun audit || true
cd "$ADMIN_REPO" && bun audit || true
cd "$WEB_REPO" && bun audit || true
```

Pass criteria:

- no untriaged high/critical advisories in runtime dependencies
- exceptions are documented with rationale, owner, and expiry date

---

## Step 3 - Lockfile and resolution hygiene

```bash
rg -n "Cargo.lock|bun.lock|bun.lockb|package-lock.json|pnpm-lock.yaml|yarn.lock" "$API_REPO" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO"
```

Check for:

- expected lockfiles are present and committed
- no mixed package manager lockfiles in same repo unless intentional

Pass criteria:

- one package manager policy per repo
- lockfiles align with manifest changes

---

## Step 4 - Dependency freshness and bloat signals

### 4.1 Rust outdated scan (if tooling available)

```bash
cd "$API_REPO" && cargo outdated || true
```

### 4.2 JS outdated scan

```bash
cd "$CLIENT_REPO" && bun outdated || true
cd "$ADMIN_REPO" && bun outdated || true
cd "$WEB_REPO" && bun outdated || true
```

Pass criteria:

- high-priority runtime deps are maintained
- very stale deps are tracked with upgrade plan

---

## Step 5 - Suspicious install/runtime script review

```bash
rg -n "preinstall|install|postinstall|prepare|prepublish|postpublish" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "package.json"
```

Review for:

- scripts that execute network calls, shell pipes, or opaque binaries
- scripts that alter files outside expected build/test flow

Pass criteria:

- install scripts are minimal, auditable, and justified

---

## Step 6 - Source trust and registry policy

```bash
rg -n "registry|source|replace-with|patch\.|git =|path =" "$API_REPO" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "Cargo.toml" -g ".cargo/config*" -g "package.json" -g "bunfig.toml"
```

Review for:

- direct git dependencies without pinning/review policy
- private registries and mirror policies
- unreviewed path/git overrides in production code paths

Pass criteria:

- dependency sources are trusted and intentional
- overrides are documented and temporary where possible

---

## Step 7 - Unused dependency cleanup candidates

Rust (if tooling available):

```bash
cd "$API_REPO" && cargo machete || true
```

TS/JS (manual indicators):

```bash
rg -n "from ['\"]" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- obvious unused deps are removed
- broad utility deps are justified by real usage

---

## Step 8 - License and policy checks (optional but recommended)

If your org requires allowlist/denylist licensing:

```bash
cd "$API_REPO" && cargo deny check licenses || true
```

JS equivalent depends on your selected license tooling.

Pass criteria:

- dependency licenses match policy
- exceptions are explicitly approved

---

## Step 9 - Correction playbook

When findings are present:

1. patch/upgrade high-severity vulnerable deps first
2. remove unused or redundant dependencies
3. reduce risky script behavior and lock source trust boundaries
4. add dependency policy checks into CI (`cargo audit`, `bun audit`, optional license checks)
5. document accepted exceptions with owner + expiry date

---

## Severity rubric

- `high`: exploitable vulnerability or untrusted dependency source in runtime path
- `medium`: stale/risky dependency posture with moderate security or maintenance risk
- `low`: hygiene or consistency issue
- `note`: process improvement opportunity

---

## Findings template

```md
### [SEVERITY] Dependency hygiene issue - <package/repo>

- **Location:** `Cargo.toml` / `package.json` / lockfile
- **Issue type:** Vulnerability / stale dep / source trust / script risk / license
- **Current state:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved / Risk accepted
```

Summary section:

```md
## Dependency hygiene sweep summary

- High issues: N
- Medium issues: N
- Low issues: N
- Accepted exceptions: N
```

---

## Related docs

- [001-security-sweep.md](./001-security-sweep.md)
- [011-migration-safety-sweep.md](./011-migration-safety-sweep.md)
- [120-configuration.md](../guides/120-configuration.md)
- [190-upgrade-compatibility.md](../guides/190-upgrade-compatibility.md)
