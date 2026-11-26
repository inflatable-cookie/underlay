#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
# Agent/proof scratch belongs on the Builds volume. Refuse when it is not
# mounted instead of falling back to $TMPDIR, $HOME or the checkout.
# Hosted GitHub Actions runners have no Builds volume, so inside Actions the
# proof uses the runner's own temp directory (contract 123, CI-only exception).
if [[ "${GITHUB_ACTIONS:-}" == true ]]; then
	if [[ -z "${RUNNER_TEMP:-}" ]]; then
		echo 'error: GITHUB_ACTIONS=true but RUNNER_TEMP is unset or empty; refusing to choose a scratch base' >&2
		exit 1
	fi
	SCRATCH_BASE="${RUNNER_TEMP}/underlay-scratch"
	mkdir -p -- "$SCRATCH_BASE"
else
	SCRATCH_BASE="/Volumes/Builds/scratch/underlay"
	if [[ ! -d "$SCRATCH_BASE" ]]; then
		echo "error: agent/proof scratch volume is unavailable: $SCRATCH_BASE" >&2
		echo 'Mount the Builds volume; do not fall back to $TMPDIR, $HOME or the checkout.' >&2
		exit 1
	fi
fi
TMP_ROOT="$(mktemp -d "$SCRATCH_BASE/consumer-conformance.XXXXXXXX")"
trap 'rm -rf -- "$TMP_ROOT"' EXIT

write_shared_fixture_files() {
	local root="$1"
	mkdir -p "$root/src"
	cat > "$root/_headers" <<'EOF'
/*
  Content-Security-Policy: default-src 'self'
*/
EOF
}

write_safe_fixture() {
	local root="$1"
	write_shared_fixture_files "$root"
	cat > "$root/src/common.rs" <<'EOF'
fn configure_cors(environment: Environment, origins: Vec<String>) -> CorsLayer {
    underlay_http::admin_cors_layer_with_headers(
        environment,
        origins,
        [http::header::HeaderName::from_static("idempotency-key")],
    )
}
EOF
	mkdir -p "$root/src/routes"
	cat > "$root/src/routes/safe.rs" <<'EOF'
pub fn build_router_with_options(options: RouterOptions) -> Router {
    let router = Router::new();
    if options.include_docs {
        router.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));
    }
    router
}
EOF
	cat > "$root/src/main.rs" <<'EOF'
fn main_router(environment: Environment) {
    let include_docs = environment.is_development();
    routes::build_router_with_options(RouterOptions {
        include_docs,
    });
    if !environment.is_development() {
        return;
    }
    routes::build_router_with_options(RouterOptions { include_docs: true });
}
EOF
	cat > "$root/src/queries.rs" <<'EOF'
pub async fn list_projects(ids: &[Uuid], pool: &PgPool) -> Result<Vec<Project>> {
    let rows = sqlx::query(r#"SELECT id FROM projects WHERE id = ANY($1::uuid[])"#)
        .bind(ids)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn list_legacy_projects(pool: &PgPool) -> Result<Vec<Project>> {
    // conformance: allow bounded-queries: one-time migration snapshot reads old rows before identifier conversion.
    let rows = sqlx::query(r#"SELECT id FROM legacy_projects"#)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
EOF
	cat > "$root/src/rows.rs" <<'EOF'
pub async fn append_audit_rows(rows: &[AuditRow], pool: &PgPool) -> Result<()> {
    // conformance: allow row-enrichment-loop: audit appends must keep arrival order.
    for row in rows {
        append_audit_entry(pool, row).await?;
    }
    Ok(())
}

pub async fn sync_manifest_rows(rows: &[ManifestRow], pool: &PgPool) -> Result<()> {
    // conformance: allow ROW-ENRICHMENT-LOOP: manifest appends must keep arrival order.
    for row in rows {
        append_manifest_entry(pool, row).await?;
    }
    Ok(())
}
EOF
}

write_unsafe_fixture() {
	local root="$1"
	write_shared_fixture_files "$root"
	cat > "$root/src/common.rs" <<'EOF'
fn configure_cors() {
    admin_cors_layer();
}
EOF
	mkdir -p "$root/src/routes"
	cat > "$root/src/routes/safe.rs" <<'EOF'
pub fn build_router_with_options(options: RouterOptions) -> Router {
    let router = Router::new();
    if options.include_docs {
        router.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));
    }
    router
}
EOF
	cat > "$root/src/routes/unsafe.rs" <<'EOF'
pub fn admin_router_with_options(options: RouterOptions) -> Router {
    let router = Router::new();
    if options.include_docs {
        router.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));
    }
    router
}

pub fn production_only_docs(environment: Environment) -> Router {
    let router = Router::new();
    if environment != Environment::Dev {
        router.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));
    }
    router
}

pub fn non_dev_non_local_docs(environment: Environment) -> Router {
    let router = Router::new();
    if environment != Environment::Local && environment != Environment::Dev {
        router.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()));
    }
    router
}
EOF
	cat > "$root/src/main.rs" <<'EOF'
fn main_router(environment: Environment) {
    let include_docs = true;
    {
        let include_docs = environment.is_development();
    }
    routes::build_router_with_options(RouterOptions {
        include_docs: environment.is_development(),
    });
    routes::admin_router_with_options(RouterOptions { include_docs });
}
EOF
	cat > "$root/src/queries.rs" <<'EOF'
const LIST_ALL_SQL: &str = r#"SELECT id FROM archived_projects"#;
const LIST_BOUNDED_SQL: &str = r#"SELECT id FROM current_projects LIMIT 20"#;

pub async fn list_projects(ids: &[Uuid], pool: &PgPool) -> Result<Vec<Project>> {
    let sample = sqlx::query(r#"SELECT id FROM projects WHERE id = $1 LIMIT 1"#)
        .bind(ids.first())
        .fetch_optional(pool)
        .await?;
	let projects = sqlx::query(r#"SELECT id, 'LIMIT 1' AS note FROM projects WHERE status IN (SELECT status FROM audit LIMIT 1)"#)
        .fetch_all(pool)
        .await?;
    Ok(projects)
}

pub async fn list_archived_projects(pool: &PgPool) -> Result<Vec<Project>> {
    let rows = sqlx::query(LIST_ALL_SQL)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn list_generic_projects(pool: &PgPool) -> Result<Vec<Project>> {
    let bounded = sqlx::query_as::<_, Project>(LIST_BOUNDED_SQL)
        .fetch_all(pool)
        .await?;
    // conformance: allow bounded-queries: a neighboring fixture query models an approved migration snapshot.
    let allowed = sqlx::query_as::<_, Project>(LIST_ALL_SQL)
        .fetch_all(pool)
        .await?;
    let rows = sqlx::query_as::<_, Project>(LIST_ALL_SQL)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn list_generic_scalar(pool: &PgPool) -> Result<Vec<Option<Vec<u8>>>> {
    let rows = sqlx::query_scalar::<_, Option<Vec<u8>>>(LIST_ALL_SQL)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn list_bounded_generic_scalar(pool: &PgPool) -> Result<Vec<Option<Vec<u8>>>> {
    let rows = sqlx::query_scalar::<_, Option<Vec<u8>>>(LIST_BOUNDED_SQL)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn list_legacy_projects(pool: &PgPool) -> Result<Vec<Project>> {
    // conformance: allow bounded-queries: one-time migration snapshot reads old rows before identifier conversion.
    let rows = sqlx::query(r#"SELECT id FROM legacy_projects"#)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
EOF
	cat > "$root/src/rows.rs" <<'EOF'
pub async fn enrich_rows(rows: &[Row], pool: &PgPool) -> Result<()> {
    for row in rows {
        let detail = fetch_detail(pool, row.id).await?;
        store_detail(pool, detail).await?;
    }
    Ok(())
}

pub async fn sync_ledger(rows: &[Row], pool: &PgPool) -> Result<()> {
    // conformance: allow row-enrichment-loop: ledger order is externally observable.
    for row in rows {
        append_ledger_entry(pool, row).await?;
    }
    Ok(())
}

pub async fn sync_ledger_unreasoned(rows: &[Row], pool: &PgPool) -> Result<()> {
    // conformance: allow row-enrichment-loop:
    for row in rows {
        append_ledger_entry(pool, row).await?;
    }
    Ok(())
}

pub fn total_weight(rows: &[Row]) -> i64 {
    let mut total = 0;
    // conformance: allow row-enrichment-loop: stale marker above an in-memory loop.
    for row in rows {
        total += row.weight;
    }
    total
}

pub async fn sync_ledger_then_enrich(rows: &[Row], pool: &PgPool) -> Result<()> {
    // conformance: allow row-enrichment-loop: ledger order is externally observable.
    for row in rows {
        append_ledger_entry(pool, row).await?;
    }
    for row in rows {
        refresh_cache(pool, row.id).await?;
    }
    Ok(())
}
EOF
	cat > "$root/src/rows_inline.rs" <<'EOF'
pub async fn enrich_pairs(pairs: &[Pair], rows: &[Row], pool: &PgPool) -> Result<()> {
    // conformance: allow row-enrichment-loop: the first loop on this line stays ordered.
    for pair in pairs { for row in rows { store_pair(pool, pair, row).await?; } }
    Ok(())
}
EOF
	cat > "$root/src/rows_unicode.rs" <<'EOF'
pub async fn enrich_unicode_rows(übers: &[Row], pool: &PgPool) -> Result<()> {
    for über in übers {
        store_detail(pool, über.id).await?;
    }
    Ok(())
}
EOF
	cat > "$root/vite.config.ts" <<'EOF'
const scoped = process.env.ENVIRONMENT; // conformance: allow ROW-ENRICHMENT-LOOP: the conformance: allow phrase here is prose, not a waiver.
export const broad = process.env.ENVIRONMENT; // conformance: allow this build config reads the deployment environment deliberately.
export const cased = process.env.ENVIRONMENT; // CONFORMANCE: ALLOW a differently cased marker is not the legacy text.
export const spaced = process.env.ENVIRONMENT; // conformance:  allow a differently spaced marker is not the legacy text.
EOF
}

if ! tr '\n' ' ' < "$REPO_ROOT/package.json" | grep -Eq '"files"[[:space:]]*:[[:space:]]*\[[[:space:]]*"ts"[[:space:]]*\]'; then
	echo "package.json must keep the checker under the published ts file set" >&2
	exit 1
fi
if ! grep -Eq '"underlay-consumer-security"[[:space:]]*:[[:space:]]*"\./ts/bin/underlay-consumer-security\.sh"' "$REPO_ROOT/package.json"; then
	echo "package.json must expose the staged checker as underlay-consumer-security" >&2
	exit 1
fi

PACKAGE_ROOT="$TMP_ROOT/package"
mkdir -p "$PACKAGE_ROOT/node_modules/.bin"
cp "$REPO_ROOT/package.json" "$PACKAGE_ROOT/package.json"
cp -R "$REPO_ROOT/ts" "$PACKAGE_ROOT/ts"
PACKAGE_BIN="$PACKAGE_ROOT/node_modules/.bin/underlay-consumer-security"
ln -s "$PACKAGE_ROOT/ts/bin/underlay-consumer-security.sh" "$PACKAGE_BIN"
if [[ -e "$PACKAGE_ROOT/scripts" ]]; then
	echo "package-like staging unexpectedly contains the checkout scripts directory" >&2
	exit 1
fi

run_both_entry_points() {
	local root="$1"
	local expected_status="$2"
	local bash_path="$3"
	local skip_checks="${4:-}"
	local bash_dir="${bash_path%/*}"
	local shell_output shell_status package_output package_status
	set +e
	shell_output=$(PATH="$bash_dir:$PATH" CONFORMANCE_SKIP="$skip_checks" "$bash_path" "$REPO_ROOT/scripts/check-consumer-conformance.sh" "$root" 2>&1)
	shell_status=$?
	package_output=$(PATH="$bash_dir:$PATH" CONFORMANCE_SKIP="$skip_checks" "$PACKAGE_BIN" "$root" 2>&1)
	package_status=$?
	set -e

	if [[ "$shell_status" -ne "$package_status" || "$shell_status" -ne "$expected_status" ]]; then
		printf 'Entry-point statuses differ or are unexpected (legacy=%s, package=%s, expected=%s)\n' \
			"$shell_status" "$package_status" "$expected_status" >&2
		printf '\n--- legacy output ---\n%s\n--- package output ---\n%s\n' "$shell_output" "$package_output" >&2
		exit 1
	fi
	if [[ "$shell_output" != "$package_output" ]]; then
		printf 'Legacy and package entry points produced different reports\n' >&2
		printf '\n--- legacy output ---\n%s\n--- package output ---\n%s\n' "$shell_output" "$package_output" >&2
		exit 1
	fi

	printf '%s' "$shell_output"
}

BASH_BINARIES=(/bin/bash)
for candidate in /opt/homebrew/bin/bash /usr/local/bin/bash; do
	if [[ -x "$candidate" ]]; then
		major_version=$("$candidate" -c 'printf "%s" "${BASH_VERSINFO[0]}"')
		if [[ "$major_version" -gt 3 ]]; then
			BASH_BINARIES+=("$candidate")
		fi
	fi
done

SAFE_ROOT="$TMP_ROOT/safe-consumer"
write_safe_fixture "$SAFE_ROOT"

UNSAFE_ROOT="$TMP_ROOT/unsafe-consumer"
write_unsafe_fixture "$UNSAFE_ROOT"

SKIP_ONLY_ROOT="$TMP_ROOT/skip-only-consumer"
write_safe_fixture "$SKIP_ONLY_ROOT"
ALL_CHECKS="env-fail-closed,db-errors,openapi-gated,seeds-gated,html-sanitized,svg-blacklist,csp-served,tracked-secrets,totp-cipher,canonical-sessions,role-hierarchy,refresh-recheck,per-row-reorder,row-enrichment-loop,bounded-queries,detail-fanout,cors-canonical,build-env-read"

for bash_path in "${BASH_BINARIES[@]}"; do
	SAFE_OUTPUT="$(run_both_entry_points "$SAFE_ROOT" 0 "$bash_path")"
	if ! grep -Eq 'All conformance checks passed\.' <<<"$SAFE_OUTPUT"; then
		echo "safe fixture did not pass the consumer checker under $bash_path" >&2
		printf '%s\n' "$SAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'ALLOW  row-enrichment-loop src/rows\.rs:[0-9]+: audit appends must keep arrival order\.' <<<"$SAFE_OUTPUT"; then
		echo "documented row-enrichment allowance was not reported with its path, line and reason under $bash_path" >&2
		printf '%s\n' "$SAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'ALLOW  row-enrichment-loop src/rows\.rs:[0-9]+: manifest appends must keep arrival order\.' <<<"$SAFE_OUTPUT"; then
		echo "a case-insensitive row-enrichment marker was not accepted the way the analyzer recognizes it under $bash_path" >&2
		printf '%s\n' "$SAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'src/rows\.rs:[0-9]+: await inside a row loop' <<<"$SAFE_OUTPUT")" -ne 0 ]]; then
		echo "a reasoned row-enrichment allowance did not cover its own loop under $bash_path" >&2
		printf '%s\n' "$SAFE_OUTPUT" >&2
		exit 1
	fi

	UNSAFE_OUTPUT="$(run_both_entry_points "$UNSAFE_ROOT" 1 "$bash_path")"
	if ! grep -Eq 'src/routes/unsafe\.rs:.*mount must be inside a development-only gate' <<<"$UNSAFE_OUTPUT"; then
		echo "unguarded cross-file OpenAPI mount was not reported precisely under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'src/rows\.rs:[0-9]+: await inside a row loop' <<<"$UNSAFE_OUTPUT")" -ne 2 ]]; then
		echo "unannotated row loops were not reported once each beside an allowed loop under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'ALLOW  row-enrichment-loop src/rows\.rs:[0-9]+: ledger order is externally observable\.' <<<"$UNSAFE_OUTPUT")" -ne 2 ]]; then
		echo "accepted row-enrichment sites were not reported with path, line and reason beside failures under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/rows\.rs:[0-9]+: row-enrichment-loop allowance needs a same-line reason' <<<"$UNSAFE_OUTPUT"; then
		echo "an empty row-enrichment reason was not a conformance failure under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/rows\.rs:[0-9]+: row-enrichment-loop allowance does not annotate the next line' <<<"$UNSAFE_OUTPUT"; then
		echo "a marker above a loop that would not be flagged was not a conformance failure under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'src/rows_inline\.rs:[0-9]+: await inside a row loop' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
		echo "a second loop header on the same line was not left visible beside its allowance under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'ALLOW  row-enrichment-loop src/rows_inline\.rs:[0-9]+: the first loop on this line stays ordered\.' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
		echo "one marker did not cover exactly one same-line loop under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'src/rows_unicode\.rs:[0-9]+: await inside a row loop' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
		echo "a Unicode-identifier row loop was not reported after the heuristic moved off ripgrep under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'vite\.config\.ts:[0-9]+:' <<<"$UNSAFE_OUTPUT")" -ne 3 ]]; then
		echo "a rule-scoped, differently cased or differently spaced marker waived build-env-read under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'vite\.config\.ts:1:const scoped' <<<"$UNSAFE_OUTPUT" ||
		! grep -Eq 'vite\.config\.ts:3:export const cased' <<<"$UNSAFE_OUTPUT" ||
		! grep -Eq 'vite\.config\.ts:4:export const spaced' <<<"$UNSAFE_OUTPUT"; then
		echo "the scoped, differently cased and differently spaced build-config rows were not reported line-locally under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if grep -Eq 'vite\.config\.ts:2:' <<<"$UNSAFE_OUTPUT"; then
		echo "the literal broad allowance line stopped waiving build-env-read under $bash_path" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/queries\.rs:.*list_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
		echo "unbounded list query was not reported beside unrelated LIMIT and allowance text" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/queries\.rs:.*list_archived_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
		echo "unbounded nongeneric module-constant SQL was not reported at its list/search call site" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/queries\.rs:.*list_generic_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
		echo "unbounded generic query_as const SQL was not reported" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if ! grep -Eq 'src/queries\.rs:.*list_generic_scalar query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
		echo "unbounded nested-generic query_scalar const SQL was not reported" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if grep -Eq 'src/queries\.rs:.*list_bounded_generic_scalar query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
		echo "query_scalar with a genuine top-level SQL LIMIT was reported" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'mount must be inside a development-only gate' <<<"$UNSAFE_OUTPUT")" -ne 3 ]]; then
		echo "safe and unsafe OpenAPI mounts were not distinguished by their own source paths" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'list_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
		echo "query-local bounds or allowances were not scoped to their own query" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi
	if [[ "$(grep -Ec 'list_generic_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
		echo "a neighboring bounded query or allowance hid generic query_as findings" >&2
		printf '%s\n' "$UNSAFE_OUTPUT" >&2
		exit 1
	fi

	SKIP_ONLY_OUTPUT="$(run_both_entry_points "$SKIP_ONLY_ROOT" 2 "$bash_path" "$ALL_CHECKS")"
	if ! grep -Eq 'Checks executed: 0; skipped: 18' <<<"$SKIP_ONLY_OUTPUT" || \
		! grep -Eq 'No conformance checks ran; coverage was not assessed\.' <<<"$SKIP_ONLY_OUTPUT"; then
		echo "all-skipped execution did not report zero coverage under $bash_path" >&2
		printf '%s\n' "$SKIP_ONLY_OUTPUT" >&2
		exit 1
	fi

	PARTIAL_SKIP_OUTPUT="$(run_both_entry_points "$SAFE_ROOT" 0 "$bash_path" "openapi-gated,bounded-queries")"
	if ! grep -Eq 'Checks executed: 16; skipped: 2' <<<"$PARTIAL_SKIP_OUTPUT" || \
		! grep -Eq 'All executed conformance checks passed; 2 check\(s\) skipped\.' <<<"$PARTIAL_SKIP_OUTPUT" || \
		grep -Eq 'All conformance checks passed\.' <<<"$PARTIAL_SKIP_OUTPUT"; then
		echo "partial skips did not retain an honest passing report under $bash_path" >&2
		printf '%s\n' "$PARTIAL_SKIP_OUTPUT" >&2
		exit 1
	fi
done

echo "Consumer security fixture proof passed through legacy and package entry points under ${#BASH_BINARIES[@]} Bash version(s)."
