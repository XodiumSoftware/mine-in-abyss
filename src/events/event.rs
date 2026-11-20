//! The trait that registers an event handler.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::events::EventPriority;

/// An event handler that registers itself with the host.
pub(crate) trait Event {
    /// The priority the host calls this handler at.
    const PRIORITY: EventPriority = EventPriority::Normal;

    /// Whether the host may call this handler off the main thread.
    ///
    /// The handlers here read and write the world, which is not thread-safe, so this is
    /// `false` unless a handler never touches it.
    const BLOCKING: bool = false;

    /// Registers this handler for the event named in its body.
    ///
    /// # Panics
    ///
    /// Panics if the host refuses, which is a wiring bug: a silently unregistered handler
    /// would mean a feature that does nothing at all.
    fn register(context: &Context);

    /// Panics because registering `handler`'s event failed.
    ///
    /// # Panics
    ///
    /// Always.
    fn fail(handler: &'static str) -> ! {
        panic!("failed to register the {handler} event handler");
    }
}
