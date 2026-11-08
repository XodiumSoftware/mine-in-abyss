//! Mechanic system for `mine-in-abyss`.
//!
//! Each gameplay feature is implemented as a module implementing the [`Mechanic`] trait.
//! Mechanics can register event handlers, commands, and permission nodes.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use serde::{Deserialize, Serialize};
use tracing::error;

/// A trait representing a plugin mechanic that can be enabled or disabled.
///
/// Mechanics may optionally expose commands, permission nodes, and event handlers,
/// all registered with the server via [`Mechanic::register`].
pub trait Mechanic {
    /// Returns `true` if the module is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns the commands provided by this mechanic.
    ///
    /// Each [`Command`] returned here will be registered with the server when
    /// [`Mechanic::register`] is called. Returns an empty vec by default.
    fn cmds(&self) -> Vec<Command> {
        vec![]
    }

    /// Returns the permission nodes required by this mechanic.
    ///
    /// All permissions returned here are registered with the server so permission
    /// plugins can grant/deny them. Enforcement happens inside each command's
    /// `CommandHandler` via `sender.has_permission(server, node)`.
    /// Returns an empty vec by default.
    fn perms(&self) -> Vec<Permission> {
        Vec::new()
    }

    /// Registers event handlers for this mechanic.
    ///
    /// Override this to call [`Mechanic::register_event`] for each event this
    /// mechanic handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers `Self` as the handler for event `T` and panics on failure.
    ///
    /// This is a thin wrapper around [`Context::register_event_handler`] that
    /// supplies the module-specific error message so individual modules don't
    /// have to repeat it.
    fn register_event<T>(&self, context: &Context, priority: EventPriority, ignore_cancelled: bool)
    where
        T: FromIntoEvent + Send + Sync + 'static,
        Self: EventHandler<T> + Default + Send + Sync + 'static,
    {
        context
            .register_event_handler::<T, _>(Self::default(), priority, ignore_cancelled)
            .expect("failed to register event handler");
    }

    /// Registers this mechanic's event handlers, permissions, and commands with the server.
    ///
    /// No-op when [`Mechanic::enabled`] returns `false`.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
        for perm in self.perms() {
            if let Err(e) = context.register_permission(&perm) {
                error!("Failed to register permission '{}': {e}", perm.node);
            }
        }
        for cmd in self.cmds() {
            context.register_command(cmd, "");
        }
    }
}

/// Top-level configuration for all mechanics (stored in `mechanics.toml`).
///
/// Each field toggles one mechanic module. Add new mechanics here.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MechanicsConfig {}
