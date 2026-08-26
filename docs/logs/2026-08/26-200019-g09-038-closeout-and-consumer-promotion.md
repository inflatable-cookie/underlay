# g09.038 Closeout And Consumer Promotion

Date: 2026-08-26
Status: complete
Roadmaps: `g09.038`; promotion gates for `g09.039`–`g09.043`

## Outcome

Underlay Reference PR
[#4](https://github.com/inflatable-cookie/underlay-reference/pull/4) merged as
`854e5ad2f9d4a7c62277447b6686bacb166516e7` from independently reviewed head
`fc83785244b1635a2d620f3ed0344bea37869079`. The reference state, migration,
health, and `TestServer` proof is complete.

The dependency gate for the five remaining consumer repairs is clear. Each
target `main` is clean and equal to `origin/main`; each declared local state
boundary was identified without shared services.

## Reference Proof

- clean-shell `effigy state apply local --yes` completed
  `reset -> structure -> dev-overlay`
- from-empty, replay, and forced-overlay-failure proof stayed inside the
  approved local `acme` database
- root/package migration tasks and active docs carry no retired `db:*` alias
- API health includes the cheap Cargo baseline
- one state-free health route slice uses `underlay_testing::TestServer`
- `effigy validate` and `effigy qa` passed with the three real test targets
- the canonical same-identity review verdict recorded no blocking findings

## Consumer Promotion Boundaries

| Roadmap | Target `main` | System | Database | Loopback binding | Persistent volume |
| --- | --- | --- | --- | --- | --- |
| `g09.039` | `3c85a5e5` | `contact-patch-dev` | `contact_patch` | `127.0.0.1:24532` | `contact-patch-dev-postgres-data` |
| `g09.040` | `240dce06` | `compli-me-dev` | `compli_me` | `127.0.0.1:22132` | `compli-me-dev-postgres-data` |
| `g09.041` | `618a5323` | `songsprout-dev` | `songsprout` | `127.0.0.1:52732` | `songsprout-dev-postgres-data` |
| `g09.042` | `153b47af` | `loophole-composer-dev` | `composer` | `127.0.0.1:58832` | `loophole-composer-dev-postgres-data` |
| `g09.043` | `3cdd5efe` | `acowtancy-dev` | `acowtancy` | `127.0.0.1:22432` | `acowtancy-dev-postgres-data` |

Every database accepted a read-only identity query as user `postgres`. Every
system reported `shared_services: []`. Songsprout required ordinary repo-owned
`effigy container up`; its allocated host port must be re-proved after any stack
recreation.

Acowtancy `effigy state plan local` resolves the pinned canonical spine digest
`sha256:79d0165f447796f23e85e0a101f5108b0ba1b954a8a44eadab62bcf970c337a9`.
An independent registry manifest fetch confirmed that digest is available.

## Planning State

- `g09.038` is complete.
- `g09.039`–`g09.043` are ready and independent.
- `g09.044` remains planned behind all five merged consumer proofs.
- `g09.045` remains planned behind repair-wave closeout.
- No run-specific triage note required disposition.

## Validation

- target `main == origin/main` and clean status checks for all five consumers
- `effigy --json system status` for all five local systems
- read-only database identity queries for all five declared databases
- Acowtancy `effigy state plan local`
- pinned Acowtancy OCI manifest fetch
- Underlay `effigy qa:docs`
- Underlay `effigy qa:northstar`
- `git diff --check`

## Next Task

Prepare, commit, and push one Northstar worker handoff per ready roadmap, then
dispatch `g09.039`–`g09.043` as five isolated parallel worker/PR lanes.
