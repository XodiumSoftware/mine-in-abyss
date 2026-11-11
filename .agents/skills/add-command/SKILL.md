---
name: add-command
description: Adds a slash command (with aliases, args, and permission wiring) to an existing mine-in-abyss feature.
---

# Add a Command

Use this skill when the user wants to add a chat command to `mine-in-abyss`.

**Discover, don't assume.** There is no `cmds()`/`perms()` registration trait in this
project, and no `pub fn register(context)` module function either. Commands implement the
`Command` trait in `src/commands/command.rs`, which owns the names, the description, the
permission node and the `register` body; `MineInAbyss::commands` calls each `register` from
one array.

## Step 0 — Read the current pattern

1. Read `src/lib.rs` (the `MineInAbyss::commands` array and the `mod commands` block).
2. Read `src/commands/abyss.rs` — the only existing command. Copy its structure.
3. Read `src/commands/command.rs` — `Command::permission_node()` builds
   `{PLUGIN_ID}:command.{primary name}` from `names()[0]`. Never hand-format the node.

## Implementation

1. Add `src/commands/<name>.rs` with:
    - `//!` module docs including a `# Usage` block and a permissions table
    - a unit struct (`#[derive(Debug, Default, Clone, Copy)]`) representing the command
    - `impl Command for <Name>` giving `names()` (primary first, then aliases), `description()`
      and `register(context)`
    - the tree built in `register`, then `context.register_command(command, &Self::permission_node())`
      and `context.register_permission(&Permission { .. })`, both using `Self::permission_node()`
    - an executor struct implementing `pumpkin_plugin_api::commands::CommandHandler`
2. Declare it in the `mod commands` block in `src/lib.rs` and add one entry to the array in
   `MineInAbyss::commands`, bumping its length:

    ```rust
    let commands: [fn(&Context); 1] = [commands::abyss::Abyss::register];
    ```

## API facts verified against `pumpkin-plugin-api` 0.2.1

- `Command::new(&names, description)` takes `&[String]`.
- `CommandNode::argument(name, &ArgumentType::String(StringType::SingleWord))`.
- `CommandNode`/`Command` builders take `self` and are **not `Clone`** — build a
  separate node per branch when a prefix is shared (see `abyss.rs`).
- `Context::register_command` takes a command and a node; `Context::register_permission` takes
  `&Permission`. Both are called from `Command::register`.
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
2. Run `just validate` and `just doc`. The release build is required: debug builds fail to
   load in the Pumpkin runtime.
3. A command can only be confirmed in a running server — state in the summary if that has not
   been done.

After finishing, summarize the changes and ask the user whether to commit.
