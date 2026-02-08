#!/usr/bin/env bash

set -euo pipefail

TARGET_DIR="${1:-rust/crates}"
WARN_LINES="${2:-500}"
FAIL_LINES="${3:-900}"

if [[ ! -d "$TARGET_DIR" ]]; then
  echo "Target directory not found: $TARGET_DIR" >&2
  exit 2
fi

echo "Checking .rs file lengths in: $TARGET_DIR"
echo "  Warn threshold: $WARN_LINES lines"
echo "  Fail threshold: $FAIL_LINES lines"
echo

warnings=""
failures=""

while IFS= read -r file; do
  lines="$(wc -l < "$file")"
  lines="${lines// /}"
  if [[ "$lines" -gt "$FAIL_LINES" ]]; then
    failures+="  ${file} (${lines} lines)"$'\n'
  elif [[ "$lines" -gt "$WARN_LINES" ]]; then
    warnings+="  ${file} (${lines} lines)"$'\n'
  fi
done < <(find "$TARGET_DIR" -name '*.rs' -not -path '*/target/*' | sort)

if [[ -n "$warnings" ]]; then
  echo "Warning — files approaching the limit (>${WARN_LINES} lines):"
  echo "$warnings"
fi

if [[ -n "$failures" ]]; then
  echo "Fail — files exceeding ${FAIL_LINES} lines:"
  echo "$failures"
  echo "Split large files per docs/roadmap/017-rust-module-splitting-roadmap.md"
  exit 1
fi

if [[ -n "$warnings" ]]; then
  echo "Pass (with warnings): no .rs files exceed ${FAIL_LINES} lines."
else
  echo "Pass: no .rs files exceed ${WARN_LINES} lines."
fi
