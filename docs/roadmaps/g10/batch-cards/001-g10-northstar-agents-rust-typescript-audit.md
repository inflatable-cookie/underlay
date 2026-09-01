# 001 - G10 Northstar AGENTS, Rust, And TypeScript Audit

Status: ready
Owner: repo maintainers
Created: 2026-09-01
Roadmap: `g10.001`
Spec: `docs/specs/northstar-instruction-and-language-quality-audit.md`
Auto-start next card: no

## Objective

Run one finding-first repository audit over Underlay's instruction surfaces,
Rust workspace, and TypeScript/Svelte foundation, then return the smallest
authorized repair set as one PR.

## Scope

- root `AGENTS.md`, `rust/AGENTS.md`, and `CLAUDE.md`;
- all workspace Rust crates, targets, and features;
- all hand-written owned TypeScript/Svelte source, tests, and tools with
  package/overlay-aware units;
- managed Northstar audit setup, recorders, focused repairs, evidence, and
  closeout surfaces.
- the bounded documentation repair that removes the deleted Loophole Composer
  repository from live Underlay fleet authority.

Exclude fixtures from production-quality claims but inventory their ownership.
Generated/vendor output, consumers, dependencies, releases, workflows, and
product work are out of scope.

## Ordered Work

1. Capture clean Git state, instruction measurements, Cargo/package/target/
   feature inventory, TS/Svelte unit map, MSRV 1.95, current toolchain, and
   explicit exclusions.
2. Record instruction dispositions. Initialize both language recorders and
   freeze disjoint units before source mutation.
3. Assess correctness, architecture, and human quality for every unit. Record
   all findings and exact-forwarder candidates before repair plans.
4. Apply only recorder-authorized repairs; extend scope before touching a
   caller, test, contract, or doc outside an owned unit.
5. Classify every deleted-repository match. Remove it from live `AGENTS.md`,
   contracts, manifests, guides, matrices, usage docs, and active sweeps;
   preserve closed roadmaps, logs, handoffs, and unrelated symbols. Reconcile
   any fleet counts or matrix assertions changed by the removal.
6. Finalize both recorders, run repository-native validation, falsify the diff
   against `g10.001`, close planning/evidence surfaces, push, and open one PR.

## Acceptance Criteria

- [ ] every instruction section, crate/target/feature, and hand-written
      TS/Svelte unit is owned or explicitly excluded;
- [ ] every normative rule and required assessment pass has a verdict per unit;
- [ ] source edits map only to pre-recorded authorized findings;
- [ ] fixtures, generated/vendor material, consumers, and protected
      instruction blocks are preserved;
- [ ] the deleted repository is absent from live fleet authority; every
      surviving match is frozen evidence or an unrelated homonym;
- [ ] MSRV, warnings, unavailable surfaces, and retained findings remain honest;
- [ ] `effigy qa`, docs/Northstar QA, Rust validation, TS/Svelte validation,
      focused tests, and `git diff --check` record actual results;
- [ ] one PR targets `main`; the worker does not merge.

## Review Oracle

Use `g10.001`. Reconcile both recorder changed-file unions with Git, then
sample a public/error/unsafe Rust boundary and an exported state/API/rendering
TypeScript or Svelte boundary. Try secret-bearing `Debug`, public export drift,
warning-as-pass, fixture-as-production, generated-source repair, and consumer
mutation counterexamples first. Also try a stale fleet count, operational guide
step, manifest row, or matrix assertion that still treats the deleted
repository as a consumer.

## Evidence Required

- instruction before/after measurements and section dispositions;
- finalized Rust and TypeScript/Svelte recorder reports and hashes;
- inventories, exclusions, changed-file attribution, focused proof, and actual
  repository QA;
- closeout under `docs/logs/2026-09/` and exact PR head.

## Stop Conditions

Use `g10.001` stop conditions. Stop if a profile, ownership, compatibility, or
public-contract decision cannot be resolved from existing authority.

## Next Task

Run this card in the dispatched worker and open a PR. Do not merge or create a
second card.
