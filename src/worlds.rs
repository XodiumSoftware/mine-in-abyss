//! Worlds for `mine-in-abyss`.
//!
//! Each layer of the Abyss lives in its own Minecraft world, so players physically
//! descend by moving between worlds. This module is responsible for creating (or
//! re-attaching to) those worlds on startup.
//!
//! World names come from [`Section::world`](crate::section::Section::world) and are
//! stable across releases: they are derived from the layer's ordinal, not its display
//! name.

use crate::section::Section;
use pumpkin_plugin_api::{Context, Dimension};
use std::cell::RefCell;
use std::collections::HashSet;
use tracing::{info, warn};

thread_local! {
    /// World names created (or attached to) by [`Worlds::setup`].
    ///
    /// Only the set of ready world names is cached; the `World` handle itself is a
    /// non-cloneable resource, so it is re-resolved from the server on demand.
    static READY: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Creates and tracks one Minecraft world per layer.
///
/// Worlds are created once during plugin load and kept for the lifetime of the server.
#[derive(Debug, Clone, Copy, Default)]
pub struct Worlds;

impl Worlds {
    /// Creates a world for every layer in `layers` and remembers it.
    ///
    /// Safe to call more than once: existing worlds are re-attached rather than
    /// duplicated, and a failure for one layer does not stop the others. Each layer's
    /// [`Section::game_rules`] are applied every time, so they cannot be lost to a world
    /// folder that was saved with different rules.
    ///
    /// Returns the number of worlds that are available after the call.
    pub fn setup(context: &Context, layers: &[Section]) -> usize {
        let server = context.get_server();

        for layer in layers {
            let name = layer.world();
            let world = server.create_world(&name, Dimension::Overworld);

            if server.has_world(&name) {
                for &(rule, value) in layer.game_rules() {
                    world.set_game_rule(rule, value);
                }
                info!("World '{name}' ready for {}", layer.name());
                READY.with(|r| r.borrow_mut().insert(name.clone()));
            } else {
                warn!("World '{name}' was not created for {}", layer.name());
            }
        }

        let count = READY.with(|r| r.borrow().len());
        info!("Worlds: {count} layer world(s) available");
        count
    }

    /// Returns whether the world called `name` was made available by [`Worlds::setup`].
    #[must_use]
    pub fn is_ready(name: &str) -> bool {
        READY.with(|r| r.borrow().contains(name))
    }

    /// Returns the number of worlds currently tracked.
    #[must_use]
    pub fn count() -> usize {
        READY.with(|r| r.borrow().len())
    }
}
