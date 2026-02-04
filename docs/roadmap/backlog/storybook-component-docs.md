# Backlog: Storybook Component Documentation

**Status**: Backlog
**Priority**: Low
**Estimated Effort**: 8-12 hours
**Source**: Deferred from Acowtancy roadmap 032 (Acme Features to Underlay)

---

## Problem Statement

Underlay's TypeScript component library (`ts/src/components/`) contains 60+ reusable Svelte components but lacks interactive documentation. Developers need to:

- Read source code to understand component props and variants
- Run a consuming app to see how components look
- Guess at available options without visual reference

This slows adoption and increases the learning curve for new developers.

---

## Proposed Solution

Set up [Storybook](https://storybook.js.org/) for the Underlay component library:

1. **Install Storybook** with Svelte 5 support
2. **Create stories** for each component showing:
   - Default state
   - All prop variants (sizes, colors, states)
   - Interactive controls for props
   - Usage examples with code snippets
3. **Configure autodocs** to generate documentation from JSDoc/TSDoc
4. **Deploy to GitHub Pages** for public access

### Components to Document (Priority Order)

**High Priority** (frequently used):
- Button, IconButton, TextButton
- TextInput, Select, Switch, DateInput
- Card, ListCard, ContentCard
- Dialog, AlertDialog, DropdownMenu
- Badge, Pill, StatusBadge
- Form, Field, FieldSet

**Medium Priority** (specialized):
- ActivityFeed, BatchActionBar
- StatCard, StatGrid
- DataTable, Pagination
- MediaPicker, FileUpload
- TabsRoot, TabsList, TabsTrigger, TabsContent

**Lower Priority** (auth-specific):
- LoginForm, RegisterForm
- TotpSetup, TotpInput
- SessionList, SecuritySettings

---

## Dependencies

- Storybook 8.x with `@storybook/sveltekit` or `@storybook/svelte`
- Svelte 5 compatibility (may need experimental support)
- GitHub Actions for deployment

---

## Success Criteria

- [ ] Storybook runs locally with `bun run storybook`
- [ ] All Button variants documented with controls
- [ ] All form components (TextInput, Select, etc.) documented
- [ ] Autodocs generates prop tables from TypeScript
- [ ] Stories deployed to GitHub Pages
- [ ] README links to live Storybook

---

## Risks & Considerations

- **Svelte 5 support**: Storybook's Svelte adapter may need updates for Svelte 5 runes
- **Maintenance burden**: Stories need updating when components change
- **Build complexity**: Adding another build tool to the monorepo
- **Time investment**: 60+ components means significant initial effort

### Mitigation

- Start with high-priority components only
- Use autodocs to reduce manual documentation
- Add story updates to component PR checklist
- Consider alternatives like Histoire (Svelte-native) if Storybook struggles

---

## Alternatives Considered

1. **Histoire** - Svelte-native alternative to Storybook, better Svelte 5 support but smaller ecosystem
2. **Manual docs site** - Custom SvelteKit site with examples, more control but more work
3. **README examples** - Code snippets in component files, no interactive preview

---

**Created**: 2026-02-04
