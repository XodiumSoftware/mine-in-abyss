---
name: update-plugin-docs
description: Updates mine-in-abyss rustdoc comments, feature doc tables, GUIDE.md, and regenerates docs after code changes.
---

# Update Plugin Docs

Use this skill after code changes in `mine-in-abyss`, or when the user asks to sync or
regenerate documentation.

## Rules (from `AGENTS.md`)

If a code change would confuse someone reading the docs, update the docs. Trigger points:

- New features, commands, config fields, or placeholders → rustdoc headers (`//!` tables)
- Changed config fields or behavior → the owning feature's docs
- Structural changes (renames, new folders, new groups) → `AGENTS.md` § Architecture Overview
  (entry point, trait table, active features, project structure)
- New/changed user-facing steps or config keys → `GUIDE.md` (and `README.md` if relevant)

## Conventions

- Module-level docs use `//!` with Markdown tables for config: `| Field | Default | Description |`.
- Document all `pub` items with `///`. Config struct fields get inline `///` comments.
- User-facing strings document their placeholders in a `## Placeholders` table.
- Commands are documented in a `## Commands` table: command, aliases, permission node, description.
- Permission nodes and namespaced ids refer to the existing `PLUGIN_ID` constant/helper — read
  it from the code rather than assuming a value.
- If the crate name and the Minecraft namespace differ, note which to use where, and check
  the current values in `Cargo.toml` / `src/lib.rs` instead of restating them from memory.

## Steps

0. **Discover the current structure first.** Read `src/lib.rs` and the registration trait so
   you describe the real tree and names — don't rely on stale notes in this skill or in
   `AGENTS.md`. If they disagree with the code, the code wins and the docs need fixing.
1. Identify the changed files (diff or ask the user).
2. Update `//!` headers and `///` comments to match current behavior, config fields, commands,
   and placeholders.
3. Check the crate-level docs in `src/lib.rs` still describe the plugin accurately.
4. Bring `AGENTS.md` § Architecture Overview in line with the code: entry point, trait method
   table, active features, project structure tree, and conventions.
5. Update `GUIDE.md` wherever it names affected features, commands, or config keys.
6. Regenerate docs and confirm they build:
   - `just doc` (or `cargo doc --no-deps --target wasm32-wasip2`)
   - Fix any broken doc links or rustdoc warnings it reports.
7. Confirm lints still pass if doc examples or imports moved:
   - `just lint` (or `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings`)

After finishing, list the files updated and ask the user whether to commit.
