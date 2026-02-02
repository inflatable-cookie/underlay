# Recipe: New Project Bootstrap Prompt

**Use when**: You are starting a brand new Underlay-based project in a fresh folder and want a clean, repeatable bootstrap flow (monorepo or multi-repo workspace).

**Example prompt**: "Bootstrap a new Underlay project from scratch"

---

## Copy/Paste Prompt (for a new LLM thread)

```text
You are a software engineering agent helping me bootstrap a brand new project using the Underlay guides.

Constraints:
- Assume the necessary repositories already exist locally.
- Expect the package folders to be mostly empty (often just README.md and maybe .gitignore).
- Do not rely on existing package.json/Cargo.toml/etc being present.
- The shared UI kit repo may or may not exist; everything else should be present or createable.
- Do not create remote repos or change billing/security posture.
- Prefer the narrowest changes; don't invent domain-specific features.
- Use the Underlay docs as the source of truth. Follow these in order and link to the exact docs you used:
  - underlay/docs/guides/010-prerequisites.md
  - underlay/docs/guides/020-project-structure.md
  - underlay/docs/guides/030-underlay-integration.md
  - then proceed only as far as needed to produce a working skeleton

Step 0: Ask me one question (and only one):
1) Is this a monorepo (single git repo) or a multi-repo workspace (separate repos side-by-side)?

After I answer, do the following:

1) Detect and summarize the current workspace
   - Treat the current working directory as the "workspace root".
   - Interpret repo boundaries based on my answer:
     - If I said monorepo: the workspace root is the single git repo.
     - If I said multi-repo workspace: each package folder under the workspace root is its own git repo; the workspace root itself is not a repo.
   - List top-level folders.
   - Detect which folders are git repos (or initialise them if missing and safe to do so).
   - Because the folders may be empty, infer intended roles using directory names first (and only then by file heuristics once you create skeleton files):
     - api: api, backend, server
     - web: web, frontend, app
     - admin: admin
     - client: client, sdk
     - ui (optional): ui, ui-kit, design-system
     - docs: docs, trellis
   - If a required folder is missing, create it. In multi-repo workspace mode, also initialise it as a git repo.

2) Symlink Underlay into the workspace root
   - Check if an "underlay" directory already exists.
   - If not present, ask me for the path to my local Underlay checkout (offer a default guess like ../underlay or ../libraries/underlay), then create a symlink named "underlay" in the workspace root.
   - Verify the symlink works by confirming underlay/package.json and underlay/docs/guides/README.md exist.
   - If this is a monorepo: add "underlay/" to the root .gitignore (create the file if needed).

3) Create/repair root-level AGENTS.md
   - If monorepo: create/update AGENTS.md in the repo root (workspace root) with:
     - a map of apps/libs folders
     - how to run checks/tests for each component
     - a clear note that Underlay is symlinked at ./underlay and is gitignored
   - If multi-repo workspace: create an AGENTS.md in the workspace root describing the workspace and reminding that each component repo has its own AGENTS.md. Do not assume the workspace root is a git repo.

4) Create/repair per-repo AGENTS.md
   - For each detected component repo (api, web, admin, client, ui if present, docs), ensure there is an AGENTS.md that:
     - states what the repo is
     - points to the Underlay guides
     - lists the relevant build/test commands
     - notes any important conventions (Rust snake_case, TS camelCase, kebab-case filenames)

5) Bootstrap the minimal skeleton (only what's missing)
   - Goal: a working "hello world" stack that compiles and typechecks.
   - Do NOT build domain features.

   5a) Rust API (if missing or empty)
     - Create a Cargo workspace and an api binary crate in the conventional structure from the Underlay guides.
     - Add Underlay Rust path deps per underlay/docs/guides/030-underlay-integration.md.
     - Implement a minimal health route and start command.

   5b) TypeScript client (if missing or empty)
     - Add @decodelabs/underlay as a file: dependency pointing at the symlinked underlay directory.
     - Add a minimal http client wrapper that matches Underlay's error envelope conventions.
     - Add a tiny typecheck script.

   5c) SvelteKit web + admin (if missing or empty)
     - Create minimal SvelteKit skeletons following underlay/docs/guides/100-frontend-web.md and underlay/docs/guides/110-admin.md.
     - Wire them to import Underlay CSS and depend on the TS client.
     - Keep routing minimal (home page + placeholder layout).

   5d) UI kit (optional)
     - If a ui-kit repo exists, wire it up as a shared Svelte package.
     - If it does not exist, skip it and make web/admin depend only on Underlay + the TS client.

6) Verification
   - Run the narrowest commands to prove the skeleton is healthy:
     - Rust: cargo build (and cargo test if there are tests)
     - Frontends/client: bun install + bun check (or the equivalent scripts you created)
   - If anything fails, fix it.

7) Output
   - At the end, print:
     - the detected project layout (monorepo vs multi-repo)
     - paths to the created/updated files
     - the exact commands to run to start api, web, admin

Important:
- Ask only the single monorepo vs multi-repo question up front.
- After that, only ask questions if you're blocked and cannot safely choose a default.
```

---

## Notes

- This prompt intentionally leans on the Underlay guides (especially `underlay/docs/guides/020-project-structure.md` and `underlay/docs/guides/030-underlay-integration.md`) so a fresh project stays aligned with existing conventions.
- In monorepo mode, symlinked `underlay/` should be ignored by git to avoid vendoring Underlay into the app repo.
