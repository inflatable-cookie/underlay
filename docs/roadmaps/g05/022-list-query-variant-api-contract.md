# g05.022 — List Query Variant API Contract

## Why

Some admin lists have named product views that are not ephemeral filters.

Example:

- Marking Hub default should show answers pending marking
- other named views should show marked, void, and all answers
- user filters should then layer on top of that named view

This should be a first-class API/UI contract.

The distinction is:

- `profile` controls payload projection
- `variant` controls the server-understood baseline query
- `filter[...]`, `sort`, `page`, and `limit` refine that baseline

## Goal

Add `variant` to the canonical list query contract and define the discovery
shape for list variants and filter metadata.

## Scope

Primary Underlay contract targets:

- `docs/contracts/115-admin-resource-api-shapes.md`
- `docs/contracts/116-canonical-collection-routes-and-query-profiles.md`
- `docs/guides/073-api-profiles-and-query-contract.md`
- `ts/src/client/query.ts`
- template query and loader types

Expected request shape:

```http
GET /v1/admin/marking/answers?profile=list&variant=pending&page=1&limit=20
GET /v1/admin/marking/answers?profile=list&variant=marked&filter[marker_id]=123
```

Avoid treating named variants as ordinary filters:

```http
GET /v1/admin/marking/answers?filter[status]=pending
```

## Contract Shape

Add `variant` to list `QueryParams`:

```ts
interface QueryParams {
  variant?: string;
  sort?: SortField[];
  filters?: FilterField[];
  page?: number;
  limit?: number;
}
```

Define list capabilities:

```ts
interface ListVariantDefinition {
  id: string;
  label: string;
  description?: string;
  tone?: "default" | "info" | "success" | "warning" | "danger";
  count?: number;
  isDefault?: boolean;
}

interface ListFilterDefinition {
  id: string;
  type: "search" | "select" | "sort";
  label: string;
  placeholder?: string;
  options?: Array<{ value: string; label: string }>;
  sortFields?: Array<{ key: string; label: string; defaultDirection?: "asc" | "desc" }>;
  variants?: string[];
}

interface ListCapabilities {
  defaultVariantId?: string;
  variants: ListVariantDefinition[];
  filters: ListFilterDefinition[];
}
```

## Discovery Policy

Preferred discovery shape:

- `GET /resource?profile=list-config`

The normal paged list remains list-shaped:

- `GET /resource?profile=list&variant=pending`

Reason:

- list payloads stay focused on rows
- config payloads can change at a different cadence
- variant counts and filter definitions are not duplicated on every list fetch

## Consumer Upgrade Impact

Expected:

- API list endpoints may gain a new optional `variant` query param
- TypeScript clients need to preserve and serialize `variant`
- existing list endpoints continue to work when `variant` is absent
- endpoints with a product-default variant should document that default

## Acceptance

- `variant` is documented as a baseline query, not a filter alias
- canonical query helpers serialize and parse `variant`
- list-capabilities wire shape is documented
- contracts state that filters and sorts layer on top of the active variant
- contracts state that `profile=list-config` returns capabilities, not list rows

## Next Task

Complete.

Landed in Underlay:

- `QueryParams` now includes `variant`
- query helper serialization, parsing, and flat-record conversion preserve
  `variant`
- template types expose `ListVariantDefinition`, `ListFilterDefinition`, and
  `ListCapabilities`
- contracts define `variant` as named baseline query state
- contracts define `profile=list-config` as the list capabilities payload

Underlay commit:

- `79fc478b` Add list query variant contract
