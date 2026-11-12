//! Event handlers for `mine-in-abyss`, and the trait that registers them.
//!
//! Each handler in `src/events/` implements [`Event`] for itself, so the event type it
//! answers, the priority it asks for and whether it blocks stay in the file that defines
//! the handler. Adding a handler therefore means adding its file, its submodule in
//! `lib.rs`, its `impl Event`, and one entry in
//! [`MineInAbyss::events`](crate::MineInAbyss).
//!
//! The submodules are declared in the `mod events` block in `lib.rs`, the same way
//! `commands`, `utils` and `world` are.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::events::EventPriority;

/// An event handler that can register itself with the host.
///
/// Implemented once per handler struct, next to the `EventHandler<T>` impl it registers,
/// so a reader sees which event a handler answers without cross-referencing a list in
/// `lib.rs`. [`MineInAbyss::events`](crate::MineInAbyss) is the only caller.
///
/// The event type cannot live here: `register_event_handler::<T, _>` is generic over the
/// concrete event, and an associated type would need a bound relating it to the
/// `EventHandler` impl that nothing can express. Each `register` therefore names its own
/// event, and only the settings *around* it are defaulted.
pub(crate) trait Event {
    /// The priority the host calls this handler at.
    ///
    /// Defaults to [`EventPriority::Normal`]. Override it only when the handler must run
    /// before or after another handler of the same event.
    const PRIORITY: EventPriority = EventPriority::Normal;

    /// Whether the host may call this handler off the main thread.
    ///
    /// Defaults to `false`: the handlers here read and write the world, which is not
    /// thread-safe. Override it only for work that never touches the world.
    const BLOCKING: bool = false;

    /// Registers this handler for the event named in its `register` body.
    ///
    /// # Panics
    ///
    /// Panics if the host refuses the registration. That is a wiring bug rather than a
    /// runtime condition — the API is unstable, and a silently unregistered handler would
    /// mean a feature that does nothing at all — so it fails loudly during `on_load`.
    fn register(context: &Context);

    /// Panics because registering `handler`'s event failed.
    ///
    /// A shared tail for every `register`: the message does not name the event, because the
    /// line that panics already does. Naming it here would mean a string per handler that
    /// nothing verifies against the type beside it.
    ///
    /// # Panics
    ///
    /// Always.
    fn fail(handler: &'static str) -> ! {
        panic!("failed to register the {handler} event handler");
    }
}
