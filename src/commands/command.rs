//! Commands for `mine-in-abyss`, and the trait that registers them.
//!
//! Each command in `src/commands/` implements [`Command`] for itself, so its name, aliases,
//! permission node and tree stay in the file that defines the executor. Adding a command
//! therefore means adding its file, its submodule in `lib.rs`, its `impl Command`, and one
//! entry in [`MineInAbyss::commands`](crate::MineInAbyss).
//!
//! The submodules are declared in the `mod commands` block in `lib.rs`, the same way
//! `events`, `utils` and `world` are.

use pumpkin_plugin_api::Context;

use crate::PLUGIN_ID;

/// A command that can register itself, and its permission node, with the host.
///
/// Implemented once per command module, next to the `CommandHandler` it builds, so a
/// reader sees what a command is gated on without cross-referencing `lib.rs`.
/// [`MineInAbyss::commands`](crate::MineInAbyss) is the only caller.
///
/// Registration is two calls, and both belong to the command: the tree and its permission
/// node are one unit, and a command whose node was never registered would be silently
/// unusable rather than visibly broken.
///
/// The names and the description live here, rather than as constants in each command's
/// file, because the permission node is derived from the primary name: writing the name
/// twice would let a renamed command keep its old node.
pub trait Command {
    /// The command's primary name followed by its aliases.
    ///
    /// The primary name is what the permission node is built from, and the whole slice is
    /// what the host accepts at the prompt.
    fn names() -> &'static [&'static str];

    /// Human-readable description, shown for the command and its permission node.
    fn description() -> &'static str;

    /// The permission node that gates this command.
    ///
    /// Built from [`PLUGIN_ID`] and the primary name, so neither the namespace nor the
    /// command segment can drift from the command itself. Derive it through this rather
    /// than formatting the node by hand.
    fn permission_node() -> String {
        format!(
            "{PLUGIN_ID}:command.{}",
            Self::names().first().copied().unwrap_or_default()
        )
    }

    /// Registers this command and its permission node.
    ///
    /// # Panics
    ///
    /// Panics if the host refuses the permission node. That is a wiring bug rather than a
    /// runtime condition — the API is unstable, and a command nobody can use would look
    /// like a missing feature — so it fails loudly during `on_load`.
    fn register(context: &Context);

    /// Panics because registering `command`'s permission node failed.
    ///
    /// A shared tail for every `register`, so the message is worded once and every command
    /// reports its failure the same way.
    ///
    /// # Panics
    ///
    /// Always.
    fn fail(command: &'static str) -> ! {
        panic!("failed to register the {command} command");
    }
}
