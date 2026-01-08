# 090 - UI Kit (Petal Pattern)

This document covers creating a shared Svelte UI kit following the Petal pattern used in Songsprout.

## UI Kit Structure

```
libs/petal/src/
├── components/          # Reusable Svelte components
│   ├── Button.svelte
│   ├── Input.svelte
│   ├── Card.svelte
│   ├── Modal.svelte
│   └── index.ts
├── patterns/            # Higher-level UI patterns
│   ├── Form.svelte
│   ├── List.svelte
│   └── ErrorBanner.svelte
├── styles/              # Design tokens and CSS
│   ├── tokens.css
│   └── global.css
├── hooks/               # Svelte hooks
│   └── index.ts
└── index.ts             # Public exports
```

## Creating Components

See code examples in `/code/090-ui-kit/`

## Next Steps

- [100-frontend-bloom.md](./100-frontend-bloom.md)
- [110-admin-greenhouse.md](./110-admin-greenhouse.md)
