# 2026-04-11 19:48:16 BST - g02.007 package consolidation closeout

`g02.007` is complete.

What landed:

- Underlay migrated its package dependency and active guide surface from the
  split Poodle package names to `@inflatable-cookie/poodle-svelte`.
- The affected consumer families were migrated and validated:
  - `underlay-reference`
  - `contact-patch`
  - `acowtancy`
  - `loophole/composer`
- `compli-me` and `songsprout` stayed clean for this package-boundary change.

Important execution note:

- The Poodle unified package needed compatibility aliases for retained caller
  names that were still live after the package merge:
  - `SearchField`
  - `TextArea`
  - `ReorderableList`
- That was treated as part of the package-consolidation migration rather than a
  new shared-surface design lane, because it prevented a second broad caller
  rewrite across the consumer set.

Validation result:

- `underlay` `effigy check`
- `underlay` `effigy qa:docs`
- `underlay` `effigy qa:northstar`
- `underlay-reference/acme-admin` `svelte-check`
- `contact-patch/cp-admin` `svelte-check`
- `acowtancy/dairy` `svelte-check`
- `acowtancy/cream` `svelte-check`
- `loophole/composer/composer-admin` `effigy check --repo .`

Residue result:

- no live split-package imports remain in Underlay or the six consumer app
  families
- remaining references are historical `acme-docs` notes only
