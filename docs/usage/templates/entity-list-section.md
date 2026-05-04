# Entity List Section

**Status:** In development (g03.004)

`EntityList` is the Level 2 section component. It is a self-contained list
surface with filters, pagination, batch actions, and empty states.

## When To Use

- Inside a detail tab that shows a child collection
- Inside a dialog that shows a picker list
- Standalone when you don't need the full page shell

## Usage

```svelte
<EntityList
  dataLoader={async (fetch, token) => loadTasks(fetch, token)}
  presentation="cards"
  renderItem={(task) => <TaskCard {task} />}
/>
```

## Props

Same as `EntityListPage` minus the page shell props (`title`, `backHref`, etc.).

## See Also

- [Entity List Page](./entity-list-page.md)
