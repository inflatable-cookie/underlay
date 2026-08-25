# Recipe: New Project Bootstrap Prompt

**Use when**: You are starting a brand new Underlay-based project in a fresh folder and want a clean, repeatable bootstrap flow.

**Example prompt**: "Bootstrap a new Underlay project from scratch"

---

## Copy/Paste Prompt (for a new LLM thread)

```text
You are a software engineering agent helping me bootstrap a brand new project using the Underlay guides.

Workspace shape (not negotiable):
- One Git repository owns the whole product workspace.
- Runtime applications live in apps/*; reusable internal packages live in packages/*.
- Docs authority is the root docs/ directory.
- The root owns one package.json, one bun.lock, one effigy.toml, one README.md, one AGENTS.md.
- Polyrepo layouts are unsupported: no nested git repos, no submodules, no symlinked Underlay checkout, no committed file: source dependencies, no libs/*.

Constraints:
- Assume the repository already exists locally, or create it with a single `git init` at the workspace root.
- Expect the package folders to be mostly empty (often just README.md).
- Do not rely on existing package.json/Cargo.toml/etc being present.
- The shared UI package is optional; everything else should be present or createable.
- Do not create remote repos or change billing/security posture.
- Prefer the narrowest changes; don't invent domain-specific features.
- Use the Underlay docs as the source of truth. Follow these in order and link to the exact docs you used:
  - underlay/docs/contracts/024-new-app-bootstrap-and-bring-up.md  (normative)
  - underlay/docs/guides/010-prerequisites.md
  - underlay/docs/guides/020-project-structure.md
  - underlay/docs/guides/030-underlay-integration.md
  - then proceed only as far as needed to produce a working skeleton

Do not ask me a layout question. There is one supported layout. Start at step 1.

1) Detect and summarize the current workspace
   - Treat the current working directory as the repository root.
   - Confirm it is a single git repository. If it is not, run `git init` once here.
   - If you find a nested .git directory, a git submodule, a child bun.lock, or a libs/ directory, STOP and report it: the folder is a pre-contract layout and needs migration, not bootstrap.
   - List top-level folders and map them onto contract roles:
     - apps/api: api, backend, server
     - apps/front: web, frontend, front, app
     - apps/admin: admin
     - packages/client: client, sdk
     - packages/ui (optional): ui, ui-kit, design-system
     - docs: docs
   - Create any missing directory under apps/ or packages/. Do not initialise it as a git repo.

2) Create the root manifest
   - Write the root package.json exactly in the contract 024 shape:
     name, private: true, packageManager: "bun@<pinned>", and an explicit workspaces array.
   - List only JavaScript packages that own a manifest. A Rust-only apps/api is NOT a workspace member.
   - Use explicit paths, not globs.

3) Wire dependencies
   - Internal packages use "workspace:*".
   - Underlay uses a pinned release tag:
     JS:    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#vX.Y.Z"
     Cargo: { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z" }
   - Poodle uses released package versions.
   - Ask me for the Underlay tag to pin if you cannot determine the current release.
   - Never write a file: dependency and never symlink or vendor Underlay into the repo.

4) Generate one lockfile
   - Run `bun install` once from the repository root and commit the resulting bun.lock.
   - Never run a per-package install.
   - If a child lockfile appears at any point, STOP and report it as in step 1. Do not delete it and do not convert the layout yourself; that is a migration, and a migration is an operator decision.

5) Create/repair the root AGENTS.md
   - Map the apps/* and packages/* directories to their roles.
   - State the Effigy-first loop (effigy tasks / health / test --plan) and the single frozen root install.
   - State that Underlay and Poodle are released dependencies, not directories in the repo.
   - Keep it lean per underlay/docs/guides/172-agents-files.md. Package-level AGENTS.md files are optional refinements only.

6) Bootstrap the minimal skeleton (only what's missing)
   - Goal: a working "hello world" stack that compiles and typechecks.
   - Do NOT build domain features.

   6a) Rust API (if missing or empty)
     - Create an app-local Cargo workspace at apps/api/Cargo.toml in the conventional crate structure from the Underlay guides.
     - Add Underlay Rust tag deps per underlay/docs/guides/030-underlay-integration.md.
     - Do not hoist the Cargo workspace to the repository root.
     - Implement a minimal health route and start command.

   6b) TypeScript client (if missing or empty)
     - Create packages/client as a workspace member with a manifest and no lockfile.
     - Add a minimal http client wrapper that matches Underlay's error envelope conventions.
     - Add a tiny typecheck script.

   6c) SvelteKit front + admin (if missing or empty)
     - Create minimal SvelteKit skeletons under apps/front and apps/admin following underlay/docs/guides/100-frontend-web.md and underlay/docs/guides/110-admin.md.
     - Depend on packages/client with "workspace:*" and on Underlay/Poodle as released dependencies.
     - Keep routing minimal (home page + placeholder layout).

   6d) UI package (optional)
     - If a shared UI package is wanted, create packages/ui as a workspace member.
     - If not, make front/admin depend only on Underlay, Poodle, and packages/client.

7) Verification
   - Run the narrowest commands to prove the skeleton is healthy:
     - Root: bun install --frozen-lockfile
     - Rust: cargo build in apps/api (and cargo test if there are tests)
     - Frontends/client: bun check (or the equivalent scripts you created)
   - Confirm there is exactly one bun.lock, at the root.
   - If anything fails, fix it.

8) Output
   - At the end, print:
     - the resolved apps/* and packages/* role map
     - paths to the created/updated files
     - the exact commands to run to start api, front, admin

Important:
- Do not ask a monorepo vs multi-repo question. There is one supported layout.
- Only ask questions if you're blocked and cannot safely choose a default (the Underlay release tag is the likely one).
```

---

## Notes

- The prompt leans on `underlay/docs/contracts/024-new-app-bootstrap-and-bring-up.md` for the normative shape and on `underlay/docs/guides/020-project-structure.md` plus `underlay/docs/guides/030-underlay-integration.md` for the procedure, so a fresh project stays aligned with the contract.
- `acowtancy` is the live proof of this layout. Mirror it when a judgement call is not covered by the guides.
- A sibling Underlay checkout is a QA and tooling convenience only. It never becomes the committed dependency shape, and nothing should be symlinked or gitignored into the app repository to fake one.
