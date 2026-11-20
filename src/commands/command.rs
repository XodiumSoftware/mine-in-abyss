//! The trait that registers a command.

use pumpkin_plugin_api::Context;

use crate::PLUGIN_ID;

/// A command that registers itself, and its permission node, with the host.
pub(crate) trait Command {
    /// The primary name followed by its aliases.
    fn names() -> &'static [&'static str];

    /// Human-readable description, shown for the command and its permission node.
    fn description() -> &'static str;

    /// The permission node that gates this command, derived from the primary name so neither
    /// the namespace nor the command segment can drift.
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
    /// Panics if the host refuses the permission node, which is a wiring bug.
    fn register(context: &Context);

    /// Panics because registering `command`'s permission node failed.
    ///
    /// # Panics
    ///
    /// Always.
    fn fail(command: &'static str) -> ! {
        panic!("failed to register the {command} command");
    }
}
