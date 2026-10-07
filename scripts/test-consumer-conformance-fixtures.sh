#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/underlay-consumer-conformance.XXXXXX")"
trap 'rm -rf -- "$TMP_ROOT"' EXIT

write_shared_fixture_files() {
	local root="$1"
	mkdir -p "$root/src"
	cat > "$root/_headers" <<'EOF'
/*
  Content-Security-Policy: default-src 'self'
*/
EOF
	cat > "$root/src/common.rs" <<'EOF'
fn configure_cors() {
    admin_cors_layer();
}
EOF
}

write_safe_fixture() {
	local root="$1"
	write_shared_fixture_files "$root"
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
    routes::build_router_with_options(RouterOptions {
        include_docs: environment.is_development(),
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
}

write_unsafe_fixture() {
	local root="$1"
	write_shared_fixture_files "$root"
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
EOF
	cat > "$root/src/main.rs" <<'EOF'
fn main_router(environment: Environment) {
    routes::build_router_with_options(RouterOptions {
        include_docs: environment.is_development(),
    });
    if environment.is_development() {
        let _debug_only = true;
    }
    routes::admin_router_with_options(RouterOptions { include_docs: true });
}
EOF
	cat > "$root/src/queries.rs" <<'EOF'
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

pub async fn list_legacy_projects(pool: &PgPool) -> Result<Vec<Project>> {
    // conformance: allow bounded-queries: one-time migration snapshot reads old rows before identifier conversion.
    let rows = sqlx::query(r#"SELECT id FROM legacy_projects"#)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
EOF
}

if ! rg -Uq '"files"\s*:\s*\[\s*"ts"\s*\]' "$REPO_ROOT/package.json"; then
	echo "package.json must keep the checker under the published ts file set" >&2
	exit 1
fi
if ! rg -q '"underlay-consumer-security"\s*:\s*"\./ts/bin/underlay-consumer-security\.sh"' "$REPO_ROOT/package.json"; then
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
	local shell_output shell_status package_output package_status
	set +e
	shell_output=$(bash "$REPO_ROOT/scripts/check-consumer-conformance.sh" "$root" 2>&1)
	shell_status=$?
	package_output=$("$PACKAGE_BIN" "$root" 2>&1)
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

SAFE_ROOT="$TMP_ROOT/safe-consumer"
write_safe_fixture "$SAFE_ROOT"
SAFE_OUTPUT="$(run_both_entry_points "$SAFE_ROOT" 0)"
if ! rg -q 'All conformance checks passed\.' <<<"$SAFE_OUTPUT"; then
	echo "safe fixture did not pass the consumer checker" >&2
	printf '%s\n' "$SAFE_OUTPUT" >&2
	exit 1
fi

UNSAFE_ROOT="$TMP_ROOT/unsafe-consumer"
write_unsafe_fixture "$UNSAFE_ROOT"
UNSAFE_OUTPUT="$(run_both_entry_points "$UNSAFE_ROOT" 1)"
if ! rg -q 'src/routes/unsafe\.rs:.*mount must be inside a development-only gate' <<<"$UNSAFE_OUTPUT"; then
	echo "unguarded cross-file OpenAPI mount was not reported precisely" >&2
	printf '%s\n' "$UNSAFE_OUTPUT" >&2
	exit 1
fi
if ! rg -q 'src/queries\.rs:.*list_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT"; then
	echo "unbounded list query was not reported beside unrelated LIMIT and allowance text" >&2
	printf '%s\n' "$UNSAFE_OUTPUT" >&2
	exit 1
fi
if [[ "$(rg -c 'mount must be inside a development-only gate' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
	echo "safe and unsafe OpenAPI mounts were not distinguished by their own source paths" >&2
	printf '%s\n' "$UNSAFE_OUTPUT" >&2
	exit 1
fi
if [[ "$(rg -c 'list_projects query uses fetch_all' <<<"$UNSAFE_OUTPUT")" -ne 1 ]]; then
	echo "query-local bounds or allowances were not scoped to their own query" >&2
	printf '%s\n' "$UNSAFE_OUTPUT" >&2
	exit 1
fi

echo "Consumer security fixture proof passed through legacy and package entry points."
