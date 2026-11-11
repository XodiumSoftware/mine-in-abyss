//! Event handlers for `mine-in-abyss`, and the trait that registers them.
//!
//! Each handler in `src/events/` implements [`Event`] for itself, so the event type it
//! answers, the priority it asks for and the message a failed registration logs all stay
//! in the file that defines the handler. Adding a handler therefore means adding its
//! file, its submodule in `lib.rs`, its `impl Event`, and one entry in
//! [`MineInAbyss::events`](crate::MineInAbyss).
//!
//! The submodules are declared in the `mod events` block in `lib.rs`, the same way
//! `commands`, `utils` and `world` are.

use pumpkin_plugin_api::Context;

/// An event handler that can register itself with the host.
///
/// Implemented once per handler struct, next to the `EventHandler<T>` impl it registers,
/// so a reader sees which event a handler answers without cross-referencing a list in
/// `lib.rs`. [`MineInAbyss::events`](crate::MineInAbyss) is the only caller.
pub trait Event {
    /// Registers this handler for the event it answers.
    ///
    /// # Panics
    ///
    /// Panics if the host refuses the registration. That is a wiring bug rather than a
    /// runtime condition — the API is unstable, and a silently unregistered handler would
    /// mean a feature that does nothing at all — so it fails loudly during `on_load`.
    fn register(context: &Context);
}
