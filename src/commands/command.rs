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

/// A command that can register itself, and its permission node, with the host.
///
/// Implemented once per command module, next to the `CommandHandler` it builds, so a
/// reader sees what a command is gated on without cross-referencing `lib.rs`.
/// [`MineInAbyss::commands`](crate::MineInAbyss) is the only caller.
///
/// Registration is two calls, and both belong to the command: the tree and its permission
/// node are one unit, and a command whose node was never registered would be silently
/// unusable rather than visibly broken.
pub trait Command {
    /// Registers this command and the permission node that gates it.
    ///
    /// # Panics
    ///
    /// Panics if the host refuses the permission node. That is a wiring bug rather than a
    /// runtime condition — the API is unstable, and a command nobody can use would look
    /// like a missing feature — so it fails loudly during `on_load`.
    fn register(context: &Context);
}
