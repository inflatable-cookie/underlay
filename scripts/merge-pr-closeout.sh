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
# inventories any worktrees still holding the head branch and prints safe
# cleanup commands. It never deletes worktrees or operator files.
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

if [[ $# -lt 1 || "$1" == "-h" || "$1" == "--help" ]]; then
  cat <<'EOF'
Usage: merge-pr-closeout.sh <pr-number> [--squash|--merge|--rebase]

Merges the PR with --delete-branch via -R so local worktree branch deletion
cannot mask a successful provider merge. Prints leftover worktree cleanup
commands when needed. Does not remove worktrees.
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

REPO="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"
HEAD_REF="$(gh pr view "$PR" -R "$REPO" --json headRefName --jq .headRefName)"
STATE="$(gh pr view "$PR" -R "$REPO" --json state --jq .state)"

if [[ "$STATE" != "MERGED" ]]; then
  set +e
  gh pr merge "$PR" "$METHOD" --delete-branch -R "$REPO"
  merge_status=$?
  set -e

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

echo "provider merge: MERGED (PR #$PR, head=$HEAD_REF, repo=$REPO)"

# Inventory worktrees still holding the head branch. Do not remove them.
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

if [[ ${#holding[@]} -eq 0 ]]; then
  echo "local cleanup: no registered worktree holds $HEAD_REF"
  if git show-ref --verify --quiet "refs/heads/$HEAD_REF"; then
    echo "local cleanup: branch still exists; from the primary checkout run:"
    echo "  git branch -D $(printf %q "$HEAD_REF")"
  else
    echo "local cleanup: local branch already absent"
  fi
  exit 0
fi

echo "local cleanup: head branch still belongs to registered worktree(s)"
for wt in "${holding[@]}"; do
  echo "  worktree: $wt"
  echo "  safe cleanup:"
  echo "    git worktree remove $(printf %q "$wt")"
  echo "    git branch -D $(printf %q "$HEAD_REF")"
done

exit 0
