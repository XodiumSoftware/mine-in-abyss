---
name: add-command
description: Adds a slash command (with aliases, args, and permission wiring) to an existing mine-in-abyss feature.
---

# Add a Command

Use this skill when the user wants to add a chat command to an existing registered feature
(whatever the current registration trait is called).

**Discover, don't assume.** Command wiring lives in the registration trait's `cmds()`/
`perms()` methods. Read the trait definition and an existing command implementation first;
if this skill disagrees with the code, trust the code and update this skill.

## Step 0 — Discover the current pattern

1. Read the registration trait (found from `src/lib.rs` `on_load`). Note the exact
   signatures of `cmds()` and `perms()` and how `register()` consumes them.
2. Find an existing command in `src/` and copy its executor style, argument reading, and
   message sending. If none exists yet, use the Pumpkin API docs below and say so.
3. Find how permission nodes are constructed. Use the existing helper/constant if there is
   one (`PLUGIN_ID`, a `default_permission`-style helper, etc.) — never hand-format nodes.

Reference: https://docs.rs/pumpkin-plugin-api

## Before Writing Code

Ask for:

- Which feature owns the command (run the `add-layer` skill first if it doesn't exist).
- Command name and aliases (e.g. `["nickname", "nick"]`).
- Argument shape: literals (`/cmd clear`) vs. typed arguments (`/cmd <name>`), and the
  argument types available in the API version in use.
- Whether one executor struct can serve all subcommands, or each needs its own.
- Permission description text.

## Implementation

1. Add the command(s) to `cmds()`, building the tree with the API's command/node builders.
2. Implement `CommandHandler` for an executor struct below the trait impl.
3. Add the matching permission to `perms()`, using the project's existing node helper.

## Important Constraints

- **`cmds()` and `perms()` are paired by index** in the trait's `register()` — the Nth
  permission belongs to the Nth command. Keep both vecs the same length and order.
- Read arguments with the API's accessor; destructure variants with or-patterns as the
  existing code does rather than assuming a single shape.
- Commands that need a player should fail cleanly (`CommandError::PermissionDenied`) for
  console senders — match the existing convention.
- Handlers return `Ok(i32)`; match the value other handlers return.
- Import types from `pumpkin_plugin_api` modules actually in use; check the crate's re-exports
  rather than guessing paths.

## Docs & Verification

1. Add a `## Commands` table to the feature's `//!` header: command, aliases, permission
   node, description.
2. Verify with the `just` recipes (or the cargo equivalents):
   - `just build`
   - `just lint` (warnings are errors)
   - `just doc`

After finishing, summarize the changes and ask the user whether to commit.
