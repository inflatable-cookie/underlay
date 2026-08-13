#!/usr/bin/env bash
#
# Underlay consumer template conformance checks.
#
# Statically verifies that a consumer's SvelteKit admin app follows the
# canonical Underlay admin template system
# (docs/contracts/110-admin-template-system.md,
#  docs/contracts/111-consumer-template-adoption-and-exception-policy.md).
#
# Run from anywhere, against the admin app root (the directory that contains
# src/routes — for repos where the admin app is a subpackage, pass the
# subpackage path):
#
#   ../underlay/scripts/check-template-conformance.sh acme-admin
#   ../underlay/scripts/check-template-conformance.sh /path/to/consumer/admin
#
# Exit code 0 = no violations, 1 = at least one violation.
# WARN-level checks (TPL05, TPL06) report but never fail the run.
#
# Skip individual checks with CONFORMANCE_SKIP="TPL01,TPL04".
#
# Per-consumer exceptions: create `.underlay-template-conformance.allow` in
# the audited root with one rule per line:
#
#   <CHECK_ID> <path-glob> [# comment]
#
#   TPL01 src/routes/(app)/account/** # account shell is a known exception
#   TPL03 src/routes/(app)/billing/+page.svelte
#
# Globs are bash-style patterns matched against the root-relative path
# (`*` crosses directory boundaries). Blank lines and lines starting with
# `#` are ignored. Suppressed matches are listed as ALLOWED in the report.
#
# Checks:
#   TPL01  hand-rolled browse page: +page.svelte/+layout.svelte importing
#          PageHeader (without a retained Entity*/System*/Media* page shell)
#          or DataTable directly from poodle
#   TPL02  bare EntityList (not EntityListPage/EntityListCard) imported from
#          @inflatable-cookie/underlay/templates inside src/routes
#   TPL03  list route imports EntityListPage directly instead of thin-mounting
#          an app-local wrapper (src/lib/lists/*)
#   TPL04  hand-rolled query state: parseQueryParams/buildQueryString from
#          @inflatable-cookie/underlay/client/query inside src/routes
#   TPL05  (WARN) onMount + API/commands client call in a +page.svelte —
#          use the template dataLoader seam instead
#   TPL06  (WARN) raw poodle ListCard import — EntityListCard is the
#          retained shell for repeated admin collection cards

set -uo pipefail

ROOT="${1:-.}"
ROOT="${ROOT%/}"
if [[ ! -d "$ROOT/src/routes" ]]; then
  echo "error: $ROOT does not look like a SvelteKit app root (no src/routes)" >&2
  exit 2
fi
ROOT=$(cd "$ROOT" && pwd)
ROUTES="$ROOT/src/routes"
ALLOW_FILE="$ROOT/.underlay-template-conformance.allow"

FAILURES=()
PASSES=()
WARNINGS=()
ALLOWED=()

SKIP=",${CONFORMANCE_SKIP:-},"

skip() {
  [[ "$SKIP" == *",$1,"* ]]
}

pass() {
  PASSES+=("$1")
}

fail() {
  FAILURES+=("$1: $2")
}

warn() {
  WARNINGS+=("$1: $2")
}

# Retained page shells whose presence means a PageHeader import is a local
# fallback/subordinate header (an allowed exception), not a hand-rolled shell.
RETAINED_SHELLS='EntityListPage|EntityDetailPage|EntityFormPage|EntityTrashPage|MediaUploadPage|MediaUploadWorkflowPage|MediaPickerWorkflow|MediaDetailWorkflowPage|MediaListPage|SystemIndexPage|SystemAuditLogListPage|SystemJobListPage|SystemJobDetailPage|SystemMediaTrashListPage|SystemScheduledTasksListPage|SystemScheduledTaskDetailPage|AdminDashboardPage|ErrorLogListPage|ErrorLogDetailPage'

SQ="'"
RE_POODLE="from\\s*[\"$SQ]@inflatable-cookie/poodle"
RE_TEMPLATES="from\\s*[\"$SQ]@inflatable-cookie/underlay/templates[\"$SQ]"
RE_CLIENT_QUERY="from\\s*[\"$SQ]@inflatable-cookie/underlay/client/query[\"$SQ]"
RE_NAMED_IMPORT='import[^;{]*\{[^}]*'

is_allowed() {
  # $1 = check id, $2 = root-relative path
  [[ -f "$ALLOW_FILE" ]] || return 1
  local id glob
  while read -r id glob _; do
    [[ -z "${id:-}" || "$id" == \#* ]] && continue
    # shellcheck disable=SC2053
    [[ "$id" == "$1" && "$2" == $glob ]] && return 0
  done < "$ALLOW_FILE"
  return 1
}

record() {
  # $1 = check id, $2 = root-relative path → ALLOWED or HITS
  if is_allowed "$1" "$2"; then
    ALLOWED+=("$1 $2")
  else
    HITS+="$2"$'\n'
  fi
}

# --------------------------------------------------------------------------
# TPL01: hand-rolled browse page (raw PageHeader shell or raw DataTable)
# --------------------------------------------------------------------------
if ! skip "TPL01"; then
  HITS=""
  while IFS= read -r f; do
    if rg -qU "${RE_NAMED_IMPORT}\\bDataTable\\b[^}]*\\}[^;]*${RE_POODLE}" "$f"; then
      record "TPL01" "${f#$ROOT/}"
    elif rg -qU "${RE_NAMED_IMPORT}\\bPageHeader\\b[^}]*\\}[^;]*${RE_POODLE}" "$f" \
      && ! rg -q "$RETAINED_SHELLS" "$f"; then
      record "TPL01" "${f#$ROOT/}"
    fi
  done < <(rg --files "$ROUTES" -g '+page.svelte' -g '+layout.svelte' 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL01 hand-rolled-browse-page"
  else
    fail "TPL01 hand-rolled-browse-page" "raw PageHeader/DataTable page composition (use Entity*/System*/Media* page shells):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# TPL02: bare EntityList section imported inside src/routes
# --------------------------------------------------------------------------
if ! skip "TPL02"; then
  HITS=""
  while IFS= read -r f; do
    record "TPL02" "${f#$ROOT/}"
  done < <(rg -lU "${RE_NAMED_IMPORT}\\bEntityList\\b[^}]*\\}[^;]*${RE_TEMPLATES}" "$ROUTES" 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL02 entity-list-in-routes"
  else
    fail "TPL02 entity-list-in-routes" "bare EntityList used as a route browse surface (use an EntityListPage-based wrapper; EntityList belongs in src/lib inline/embed sections):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# TPL03: list route mounts EntityListPage directly instead of a wrapper
# --------------------------------------------------------------------------
if ! skip "TPL03"; then
  HITS=""
  while IFS= read -r f; do
    if rg -qU "${RE_NAMED_IMPORT}\\bEntityListPage\\b[^}]*\\}[^;]*${RE_TEMPLATES}" "$f"; then
      record "TPL03" "${f#$ROOT/}"
    fi
  done < <(rg --files "$ROUTES" -g '+page.svelte' 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL03 list-route-wrapper-bypass"
  else
    fail "TPL03 list-route-wrapper-bypass" "list route imports EntityListPage directly (thin-mount an app-local wrapper such as \$lib/lists/*):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# TPL04: hand-rolled query state in routes
# --------------------------------------------------------------------------
if ! skip "TPL04"; then
  HITS=""
  while IFS= read -r f; do
    record "TPL04" "${f#$ROOT/}"
  done < <(rg -lU "${RE_NAMED_IMPORT}\\b(parseQueryParams|buildQueryString)\\b[^}]*\\}[^;]*${RE_CLIENT_QUERY}" "$ROUTES" 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL04 hand-rolled-query-state"
  else
    fail "TPL04 hand-rolled-query-state" "parseQueryParams/buildQueryString in routes (use createPageListQueryState from @inflatable-cookie/underlay/patterns):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# TPL05 (WARN): client-side onMount data fetching in admin pages
# --------------------------------------------------------------------------
if ! skip "TPL05"; then
  HITS=""
  while IFS= read -r f; do
    if rg -q '\bonMount\s*\(' "$f" \
      && rg -q "@api-client|\\b\\w+Commands\\b|apiClient|\\\$lib/utils/api" "$f"; then
      record "TPL05" "${f#$ROOT/}"
    fi
  done < <(rg --files "$ROUTES" -g '+page.svelte' 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL05 onmount-data-fetching"
  else
    warn "TPL05 onmount-data-fetching" "onMount + API/commands client call in a page (prefer the template dataLoader seam):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# TPL06 (WARN): raw poodle ListCard instead of EntityListCard
# --------------------------------------------------------------------------
if ! skip "TPL06"; then
  HITS=""
  while IFS= read -r f; do
    record "TPL06" "${f#$ROOT/}"
  done < <(rg -lU "${RE_NAMED_IMPORT}\\bListCard\\b[^}]*\\}[^;]*${RE_POODLE}" "$ROOT/src" 2>/dev/null)
  if [[ -z "$HITS" ]]; then
    pass "TPL06 raw-list-card"
  else
    warn "TPL06 raw-list-card" "raw poodle ListCard import (EntityListCard is the retained shell for repeated admin collection cards):\n$HITS"
  fi
fi

# --------------------------------------------------------------------------
# Report
# --------------------------------------------------------------------------
echo "Template conformance report for: $ROOT"
echo

for p in "${PASSES[@]}"; do
  echo "  PASS  $p"
done

if [[ ${#WARNINGS[@]} -gt 0 ]]; then
  echo
  for w in "${WARNINGS[@]}"; do
    echo -e "  WARN  $w"
  done
fi

if [[ ${#ALLOWED[@]} -gt 0 ]]; then
  echo
  for a in "${ALLOWED[@]}"; do
    echo "  ALLOWED  $a"
  done
fi

if [[ ${#FAILURES[@]} -gt 0 ]]; then
  echo
  for f in "${FAILURES[@]}"; do
    echo -e "  FAIL  $f"
  done
  echo
  echo "${#FAILURES[@]} template conformance violation(s) found."
  exit 1
fi

echo
echo "All template conformance checks passed."
exit 0
