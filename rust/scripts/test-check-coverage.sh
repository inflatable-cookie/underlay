#!/usr/bin/env bash
# Proves the coverage wrapper's Oxide dispatch with fake tools only.
#
# It never runs Tarpaulin, Cargo or Oxide for real: a stub `oxide` records its
# argv and replays a scripted stdout/stderr/exit, so the check covers the
# `oxide exec -- cargo tarpaulin …` route, output passthrough, threshold
# handling, and the missing-`cargo-tarpaulin` refusal.
set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
WRAPPER="$REPO_ROOT/rust/scripts/check-coverage.sh"
SCRATCH_BASE="/Volumes/Builds/scratch/underlay"

if [[ ! -d "$SCRATCH_BASE" ]]; then
	printf 'error: agent/proof scratch volume is unavailable: %s\n' "$SCRATCH_BASE" >&2
	printf 'Mount the Builds volume; do not fall back to $TMPDIR, $HOME or the checkout.\n' >&2
	exit 1
fi

WORK="$(mktemp -d "$SCRATCH_BASE/coverage-route.XXXXXXXX")"
trap 'rm -rf -- "$WORK"' EXIT

fail() {
	printf 'coverage route check: %s\n' "$1" >&2
	shift
	for line in "$@"; do
		printf '%s\n' "$line" >&2
	done
	exit 1
}

write_fake_oxide() {
	local target="$1"
	cat >"$target" <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
printf '%s\n' "$@" >"${FAKE_ARGV_FILE:?}"
if [[ -n "${FAKE_STDOUT:-}" ]]; then
	printf '%s\n' "$FAKE_STDOUT"
fi
if [[ -n "${FAKE_STDERR:-}" ]]; then
	printf '%s\n' "$FAKE_STDERR" >&2
fi
exit "${FAKE_EXIT:-0}"
EOF
	chmod +x "$target"
}

write_fake_tarpaulin() {
	local target="$1"
	cat >"$target" <<'EOF'
#!/usr/bin/env bash
# Presence-only stub: the wrapper checks for this binary, it never runs it.
exit 0
EOF
	chmod +x "$target"
}

case_dir() {
	local name="$1"
	local with_tarpaulin="$2"
	local dir="$WORK/$name"
	mkdir -p "$dir/bin"
	write_fake_oxide "$dir/bin/oxide"
	if [[ "$with_tarpaulin" == yes ]]; then
		write_fake_tarpaulin "$dir/bin/cargo-tarpaulin"
	fi
	printf '%s\n' "$dir"
}

# Runs the wrapper with only the fake bin dir plus system utilities on PATH, so
# a stray real Cargo or Oxide cannot answer the dispatch.
run_wrapper() {
	local dir="$1"
	local status
	set +e
	PATH="$dir/bin:/usr/bin:/bin" \
		FAKE_ARGV_FILE="$dir/argv" \
		FAKE_STDOUT="${FAKE_STDOUT:-}" \
		FAKE_STDERR="${FAKE_STDERR:-}" \
		FAKE_EXIT="${FAKE_EXIT:-0}" \
		"$WRAPPER" >"$dir/out" 2>"$dir/err"
	status=$?
	set -e
	printf '%s\n' "$status"
}

assert_status() {
	local dir="$1"
	local expected="$2"
	local actual="$3"
	if [[ "$actual" != "$expected" ]]; then
		fail "expected exit $expected, got $actual" \
			"--- stdout ---" "$(cat "$dir/out")" \
			"--- stderr ---" "$(cat "$dir/err")"
	fi
}

assert_contains() {
	local dir="$1"
	local file="$2"
	local needle="$3"
	if ! grep -Fq -- "$needle" "$dir/$file"; then
		fail "$file is missing: $needle" \
			"--- stdout ---" "$(cat "$dir/out")" \
			"--- stderr ---" "$(cat "$dir/err")"
	fi
}

assert_not_contains() {
	local dir="$1"
	local file="$2"
	local needle="$3"
	if grep -Fq -- "$needle" "$dir/$file"; then
		fail "$file unexpectedly contains: $needle" \
			"--- $file ---" "$(cat "$dir/$file")"
	fi
}

assert_dispatch() {
	local dir="$1"
	local expected=$'exec\n--\ncargo\ntarpaulin\n--workspace\n--out\nXml'
	local actual
	if [[ ! -f "$dir/argv" ]]; then
		fail "oxide was not invoked" \
			"--- stderr ---" "$(cat "$dir/err")"
	fi
	actual="$(cat "$dir/argv")"
	if [[ "$actual" != "$expected" ]]; then
		fail "oxide argv mismatch" \
			"--- expected ---" "$expected" \
			"--- actual ---" "$actual"
	fi
}

if command -v cargo >/dev/null 2>&1 && PATH="/usr/bin:/bin" command -v cargo >/dev/null 2>&1; then
	fail "a real cargo is reachable on the system PATH; this check must not expose it"
fi

# 1. Oxide dispatch, successful run, overall coverage above the threshold.
dir="$(case_dir success yes)"
FAKE_STDOUT=""
FAKE_STDERR="54.01% coverage, 606/1122 lines covered"
FAKE_EXIT=0
status="$(run_wrapper "$dir")"
assert_status "$dir" 0 "$status"
assert_dispatch "$dir"
assert_contains "$dir" out "Overall coverage: 54.01% (min: 50%)"
assert_contains "$dir" out "Coverage thresholds passed."

# 2. Coverage below the threshold still fails the wrapper.
dir="$(case_dir below-threshold yes)"
FAKE_STDOUT=""
FAKE_STDERR="40.00% coverage, 448/1122 lines covered"
FAKE_EXIT=0
status="$(run_wrapper "$dir")"
assert_status "$dir" 1 "$status"
assert_dispatch "$dir"
assert_contains "$dir" err "Coverage check failed: 40.00% < 50%"

# 3. Tool failure: exit status is normalised to 1 and both streams survive.
dir="$(case_dir tool-failure yes)"
FAKE_STDOUT="tarpaulin stdout line"
FAKE_STDERR="tarpaulin stderr line"
FAKE_EXIT=7
status="$(run_wrapper "$dir")"
assert_status "$dir" 1 "$status"
assert_dispatch "$dir"
assert_contains "$dir" err "tarpaulin stdout line"
assert_contains "$dir" err "tarpaulin stderr line"

# 4. Unparsable output fails loudly and echoes what the tool said.
dir="$(case_dir unparsable yes)"
FAKE_STDOUT=""
FAKE_STDERR="no summary here"
FAKE_EXIT=0
status="$(run_wrapper "$dir")"
assert_status "$dir" 1 "$status"
assert_dispatch "$dir"
assert_contains "$dir" err "Failed to parse tarpaulin output"
assert_contains "$dir" err "no summary here"

# 5. Missing cargo-tarpaulin refuses before any Oxide dispatch.
dir="$(case_dir missing-tarpaulin no)"
FAKE_STDOUT=""
FAKE_STDERR=""
FAKE_EXIT=0
status="$(run_wrapper "$dir")"
assert_status "$dir" 1 "$status"
assert_contains "$dir" err "cargo-tarpaulin not found"
assert_contains "$dir" err "oxide install cargo-tarpaulin"
if [[ -f "$dir/argv" ]]; then
	fail "oxide ran even though cargo-tarpaulin is missing" \
		"--- argv ---" "$(cat "$dir/argv")"
fi

printf 'Coverage route check passed: Oxide dispatch, output, threshold and missing-tool behaviour.\n'
