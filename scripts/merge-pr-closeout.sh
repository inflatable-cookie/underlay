#!/usr/bin/env bash
#
# Merge a GitHub PR without treating worktree-local branch deletion as merge
# failure.
#
# `gh pr merge --delete-branch` deletes the remote branch, then tries to delete
# the local branch. When the head branch is checked out in a registered
# worktree, that local delete fails and gh exits 1 even though the provider
# merge already succeeded.
#
# This wrapper:
# 1. Captures the reviewed head OID before merge
# 2. Merges with `--match-head-commit` and `-R` (skip local branch delete)
# 3. Verifies the merged PR still records that exact reviewed OID
# 4. Suggests destructive local cleanup only when the local tip matches that OID
#
# Usage:
#   ./scripts/merge-pr-closeout.sh <pr-number> [--squash|--merge|--rebase]
#
# Equivalent raw flags (from any checkout of the repo):
#   REVIEWED=$(gh pr view <n> -R OWNER/REPO --json headRefOid --jq .headRefOid)
#   gh pr merge <n> --squash --delete-branch --match-head-commit "$REVIEWED" -R OWNER/REPO
#
# Exit 0 when the reviewed head merged successfully. Exit 1 when the provider
# merge did not land on that exact head. Remaining local worktree cleanup is
# reported separately and does not fail the command once merge is verified.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CLEANUP_CLI="$REPO_ROOT/scripts/lib/merge-pr-closeout-cleanup-cli.ts"

if [[ $# -lt 1 || "$1" == "-h" || "$1" == "--help" ]]; then
  cat <<'EOF'
Usage: merge-pr-closeout.sh <pr-number> [--squash|--merge|--rebase]

Captures the reviewed head OID, merges with --match-head-commit and -R, verifies
the merged PR still records that OID, then prints local cleanup only when the
local tip matches. Does not remove worktrees.
EOF
  exit 0
fi

PR="$1"
shift

METHOD="--squash"
if [[ $# -gt 0 ]]; then
  case "$1" in
    --squash|--merge|--rebase)
      METHOD="$1"
      shift
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "expected --squash, --merge, or --rebase" >&2
      exit 2
      ;;
  esac
fi

if [[ $# -gt 0 ]]; then
  echo "error: unexpected arguments: $*" >&2
  exit 2
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh is required" >&2
  exit 2
fi

if ! command -v git >/dev/null 2>&1; then
  echo "error: git is required" >&2
  exit 2
fi

if ! command -v bun >/dev/null 2>&1; then
  echo "error: bun is required for local cleanup planning" >&2
  exit 2
fi

REPO="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"
HEAD_REF="$(gh pr view "$PR" -R "$REPO" --json headRefName --jq .headRefName)"
REVIEWED_OID="$(gh pr view "$PR" -R "$REPO" --json headRefOid --jq .headRefOid)"
STATE="$(gh pr view "$PR" -R "$REPO" --json state --jq .state)"

if [[ -z "$HEAD_REF" || -z "$REVIEWED_OID" ]]; then
  echo "error: could not resolve PR headRefName/headRefOid for #$PR" >&2
  exit 1
fi

if [[ "$STATE" != "MERGED" ]]; then
  set +e
  gh pr merge "$PR" "$METHOD" --delete-branch --match-head-commit "$REVIEWED_OID" -R "$REPO"
  merge_status=$?
  set -e

  HEAD_REF="$(gh pr view "$PR" -R "$REPO" --json headRefName --jq .headRefName)"
  OBSERVED_OID="$(gh pr view "$PR" -R "$REPO" --json headRefOid --jq .headRefOid)"
  STATE="$(gh pr view "$PR" -R "$REPO" --json state --jq .state)"

  if ! bun "$CLEANUP_CLI" verify-reviewed-head \
    --reviewed-oid "$REVIEWED_OID" \
    --observed-oid "$OBSERVED_OID" \
    --state "$STATE"
  then
    if [[ "$merge_status" -ne 0 ]]; then
      echo "error: gh merge exited $merge_status; reviewed head was not merged" >&2
    fi
    exit 1
  fi

  if [[ "$merge_status" -ne 0 ]]; then
    echo "note: gh exited $merge_status after verified provider merge; treating local cleanup separately"
  fi
else
  OBSERVED_OID="$(gh pr view "$PR" -R "$REPO" --json headRefOid --jq .headRefOid)"
  if ! bun "$CLEANUP_CLI" verify-reviewed-head \
    --reviewed-oid "$REVIEWED_OID" \
    --observed-oid "$OBSERVED_OID" \
    --state "$STATE"
  then
    exit 1
  fi
  echo "PR #$PR is already MERGED at reviewed head $REVIEWED_OID"
fi

echo "provider merge: MERGED (PR #$PR, head=$HEAD_REF, reviewedOid=$REVIEWED_OID, repo=$REPO)"

holding=()
while IFS= read -r line; do
  holding+=("$line")
done < <(
  git worktree list --porcelain | awk -v branch="refs/heads/$HEAD_REF" '
    BEGIN { path="" }
    /^worktree / { path=$2; for (i=3; i<=NF; i++) path=path" "$i }
    /^branch / && $2 == branch { print path }
  '
)

LOCAL_TIP=""
if git show-ref --verify --quiet "refs/heads/$HEAD_REF"; then
  LOCAL_TIP="$(git rev-parse "refs/heads/$HEAD_REF")"
fi

cleanup_args=(
  cleanup
  --head-ref "$HEAD_REF"
  --provider-oid "$REVIEWED_OID"
  --local-tip "$LOCAL_TIP"
)
for wt in "${holding[@]+"${holding[@]}"}"; do
  cleanup_args+=(--worktree "$wt")
done

bun "$CLEANUP_CLI" "${cleanup_args[@]}"
exit 0
