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
# This wrapper always passes `-R owner/repo` so gh skips local branch deletion
# while still deleting the remote branch. After a successful provider merge it
# compares the reviewed provider head OID to the local branch tip and only then
# prints destructive cleanup commands. Divergent tips require manual inspection.
# It never deletes worktrees or operator files.
#
# Usage:
#   ./scripts/merge-pr-closeout.sh <pr-number> [--squash|--merge|--rebase]
#
# Equivalent raw flags (from any checkout of the repo):
#   gh pr merge <n> --squash --delete-branch -R OWNER/REPO
#
# Exit 0 when the PR is MERGED. Exit 1 when the provider merge did not land.
# Remaining local worktree cleanup is reported separately and does not fail
# the command.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CLEANUP_CLI="$REPO_ROOT/scripts/lib/merge-pr-closeout-cleanup-cli.ts"

if [[ $# -lt 1 || "$1" == "-h" || "$1" == "--help" ]]; then
  cat <<'EOF'
Usage: merge-pr-closeout.sh <pr-number> [--squash|--merge|--rebase]

Merges the PR with --delete-branch via -R so local worktree branch deletion
cannot mask a successful provider merge. Prints leftover worktree cleanup
commands only when the local tip matches the provider head OID. Does not
remove worktrees.
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
PROVIDER_OID="$(gh pr view "$PR" -R "$REPO" --json headRefOid --jq .headRefOid)"
STATE="$(gh pr view "$PR" -R "$REPO" --json state --jq .state)"

if [[ -z "$HEAD_REF" || -z "$PROVIDER_OID" ]]; then
  echo "error: could not resolve PR headRefName/headRefOid for #$PR" >&2
  exit 1
fi

if [[ "$STATE" != "MERGED" ]]; then
  set +e
  gh pr merge "$PR" "$METHOD" --delete-branch -R "$REPO"
  merge_status=$?
  set -e

  HEAD_REF="$(gh pr view "$PR" -R "$REPO" --json headRefName --jq .headRefName)"
  PROVIDER_OID="$(gh pr view "$PR" -R "$REPO" --json headRefOid --jq .headRefOid)"
  STATE="$(gh pr view "$PR" -R "$REPO" --json state --jq .state)"

  if [[ "$STATE" != "MERGED" ]]; then
    echo "error: provider merge did not complete for PR #$PR (state=$STATE, gh_exit=$merge_status)" >&2
    exit 1
  fi

  if [[ "$merge_status" -ne 0 ]]; then
    echo "note: gh exited $merge_status after provider merge; treating local cleanup separately"
  fi
else
  echo "PR #$PR is already MERGED"
fi

echo "provider merge: MERGED (PR #$PR, head=$HEAD_REF, headOid=$PROVIDER_OID, repo=$REPO)"

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
  --head-ref "$HEAD_REF"
  --provider-oid "$PROVIDER_OID"
  --local-tip "$LOCAL_TIP"
)
for wt in "${holding[@]+"${holding[@]}"}"; do
  cleanup_args+=(--worktree "$wt")
done

bun "$CLEANUP_CLI" "${cleanup_args[@]}"
exit 0
