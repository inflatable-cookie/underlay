#!/usr/bin/env bash

set -euo pipefail

CSV_FILE="${1:-docs/roadmap/016-compatibility-adapters.csv}"

if [[ ! -f "$CSV_FILE" ]]; then
  echo "Compatibility inventory not found: $CSV_FILE" >&2
  exit 2
fi

today="$(date +%F)"
expired=0

while IFS=, read -r repo path compat_kind sunset_date owner status notes; do
  if [[ "$repo" == "repo" ]]; then
    continue
  fi

  if [[ ! "$sunset_date" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    echo "Invalid sunset_date format for $repo:$path -> $sunset_date" >&2
    expired=1
    continue
  fi

  if [[ "$status" == "removed" ]]; then
    continue
  fi

  if [[ "$sunset_date" < "$today" ]]; then
    echo "Expired compatibility adapter: $repo:$path (sunset $sunset_date, owner $owner, kind $compat_kind)" >&2
    expired=1
  fi
done < "$CSV_FILE"

if [[ "$expired" -ne 0 ]]; then
  echo "Fail: one or more compatibility adapters have passed their sunset date." >&2
  exit 1
fi

echo "Pass: all active compatibility adapters are within sunset window."
