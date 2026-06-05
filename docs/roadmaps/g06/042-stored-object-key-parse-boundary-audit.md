# g06.042 - Stored Object Key Parse-Boundary Audit

## Why

Generated media object keys now use `BlobObjectKey` through storage helpers and
typed adapter extension methods. Stored media rows still expose object keys as
raw strings.

Before changing repository/domain row types, identify the narrow parse
boundaries where database-loaded object keys should become typed.

## Goal

Audit stored object-key flows and decide where Underlay should parse database
strings into `BlobObjectKey`.

## Scope

In scope:

- audit media repository/domain row object-key fields
- audit public URL, deletion, rendition, and download flows that load keys from
  the database
- classify parse-at-row, parse-at-DTO, parse-at-adapter, or retain-raw options
- decide whether `MediaVersion` / `MediaRendition` fields should become typed
  in Underlay
- classify consumer impact

Out of scope:

- changing database column types
- changing blob adapter trait signatures
- changing persisted object-key values
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- stored object-key use sites are known
- typed row-field impact is classified
- next implementation batch is concrete if change is warranted
- contracts reflect the chosen parse boundary

## Consumer Upgrade Impact

Expected impact: decision first.

Any implementation that changes media domain row field types is likely breaking
for consumers and must be rolled out with the six-app family.

## Current State

`g06.042` is next after `g06.041`.

## Next Task

Execute `g06.042`: stored object-key parse-boundary audit.
