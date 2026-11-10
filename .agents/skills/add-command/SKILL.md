---
name: add-command
description: Adds a slash command (with aliases, args, and permission wiring) to an existing mine-in-abyss feature.
---

# Add a Command

Use this skill when the user wants to add a chat command to `mine-in-abyss`.

**Discover, don't assume.** There is no `cmds()`/`perms()` registration trait in this
project (an earlier version of this skill claimed there was). Commands are plain
modules with a `register(context)` function called from `MineInAbyss::on_load`.

## Step 0 — Read the current pattern

1. Read `src/lib.rs` (`on_load`, the `mod commands` block).
2. Read `src/commands/abyss.rs` — the only existing command. Copy its structure.
3. Build permission nodes with `abyss::permission_node()`-style helpers derived from
   `PLUGIN_ID`; never hand-format the namespace.

## Implementation

1. Add `src/commands/<name>.rs` with:
    - `//!` module docs including a `# Usage` block and a permissions table
    - `NAMES` (primary name first, then aliases)
    - a `permission_node()` helper built from `PLUGIN_ID`
    - `pub fn register(context: &Context)` that builds the tree, calls
      `context.register_command(command, &permission_node())` and
      `context.register_permission(&Permission { .. })`
    - an executor struct implementing
      `pumpkin_plugin_api::commands::CommandHandler`
2. Declare it in the `mod commands` block in `src/lib.rs` and call
   `commands::<name>::register(&context)` from `on_load`.

## API facts verified against `pumpkin-plugin-api` 0.2.1

- `Command::new(&names, description)` takes `&[String]`.
- `CommandNode::argument(name, &ArgumentType::String(StringType::SingleWord))`.
- `CommandNode`/`Command` builders take `self` and are **not `Clone`** — build a
  separate node per branch when a prefix is shared (see `abyss.rs`).
- `Context::register_command` takes `&str`; `Context::register_permission` takes
  `&Permission`.
- `CommandSender::has_permission_level` uses
  `wit::pumpkin::plugin::command::PermissionLevel`, **not** the `permission` interface's
  same-named enum.
- `ConsumedArgs::get_value` returns `Arg`; match `Arg::Simple(..)` and prefer
  `let ... else` (pedantic clippy rejects the `match` form).
- `TextComponent::text(..)` exists; there is no `MiniMessage` parser.

## Constraints

- Guard commands that act on a player: `sender.as_player()` is `None` for console.
- Send user-facing failures with `sender.send_error(..)` and return `Ok(0)` rather than
  `Err`, so the failure reads as a message instead of a command error.
- Return `Ok(1)` on success, matching `abyss.rs`.

## Docs & Verification

1. Add the command to the `### Commands` table in `AGENTS.md`.
2. Verify: `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W
clippy::pedantic -D warnings`, `cargo build --target wasm32-wasip2`, `cargo doc
--no-deps --target wasm32-wasip2`.

After finishing, summarize the changes and ask the user whether to commit.
