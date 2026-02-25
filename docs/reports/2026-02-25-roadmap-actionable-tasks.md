# Underlay Roadmap Actionable Tasks (2026-02-25)

This report captures the non-stale, still-open work after a full roadmap sweep.

## Current Roadmap Status Snapshot

- Roadmaps with open work: `015`, `016`
- Completed roadmaps: all others in `docs/roadmap/*.md`
- Index alignment updated in `docs/roadmap/README.md` to: `21 complete / 2 in progress / 0 not started`

## Open Work That Is Actually Actionable Now

## Priority 1 - Close Remaining In-Progress Roadmaps (`015`, `016`)

1. Capture production-like error-log evidence to close `015` success gate:
   - reduce `handler_context` null-rate from current `71.43% (5/7)` on Farmyard
   - show at least one real incident with materially faster diagnosis from structured context
2. Finalize snake_case runtime validation evidence for `016`:
   - sample key API responses (completed for Songsprout + Acowtancy)
   - confirm error-log/job-payload JSON keys (completed for Songsprout + Acowtancy)
   - verify/remove remaining temporary compatibility adapters

## Priority 2 - Validation Closure Sweep (`015`/`016`)

1. Re-run the error-reporting validation sequence and archive before/after evidence.
2. Run post-reset `snake_case` runtime checks and remove final compatibility adapters.
3. Execute the combined closure command from `underlay`:
   - `scripts/roadmap-015-016-closure.sh`
   - optional runtime evidence mode:
     - `ACOWTANCY_API_BASE_URL=http://0.0.0.0:40001 scripts/roadmap-015-016-closure.sh --run-runtime`
4. Current baseline status:
   - non-runtime closure sweep is green (`docs/reports/2026-02-25-roadmap-015-016-closure-sweep.md`)
   - runtime smoke path is green, but `015` metric gate remains open (`handler_context` null-rate `85.71%`, 24h)
   - remaining work is null-rate reduction evidence + `016` cutover cleanup

## Not Actionable / Stale (Removed from Active Focus)

1. Roadmaps marked complete but still historically verbose in older sections are treated as archival context, not active backlog.
2. Completed phases with follow-up evidence gates remain open only in the top-level active sections (`015`, `016`), not as broad implementation work.

## Recommended Next Batch

1. Run one combined closure sweep for error reporting evidence (`015`) and JSON naming cutover checks (`016`).
2. Remove/verify final compatibility adapters and confirm no runtime naming mismatches after reset.
3. Update roadmap index and actionable report once the two remaining gates are closed.
