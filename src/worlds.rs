//! World manager for `mine-in-abyss`.
//!
//! Each stratum of the Abyss lives in its own Minecraft world, so players physically
//! descend a layer by moving between worlds. This module is responsible for creating
//! (or re-attaching to) those worlds on startup.
//!
//! | World name                      | Stratum |
//! | ------------------------------- | ------- |
//! | `mine_in_abyss_first`           | First   |
//! | `mine_in_abyss_second`          | Second  |
//! | ...                             | ...     |
//!
//! World names come from [`Depth::world_name`] and are stable across releases: they
//! are derived from the stratum ordinal, not its display name.

use crate::layers::layer::Depth;
use pumpkin_plugin_api::{Context, Dimension, World};
use std::cell::RefCell;
use std::collections::HashSet;
use tracing::{info, warn};

thread_local! {
    /// Strata whose world was created (or attached to) by [`WorldManager::setup`].
    ///
    /// Only the set of ready strata is cached; the `World` handle itself is a
    /// non-cloneable resource, so it is re-resolved on demand via
    /// [`WorldManager::world`].
    static READY: RefCell<HashSet<Depth>> = RefCell::new(HashSet::new());
}

/// Creates and tracks one Minecraft world per Abyss stratum.
///
/// Worlds are created once during plugin load and kept for the lifetime of the
/// server; callers look them up with [`WorldManager::world`] rather than creating
/// them on demand.
#[derive(Debug, Clone, Copy, Default)]
pub struct WorldManager;

impl WorldManager {
    /// Creates a world for every stratum and remembers it.
    ///
    /// Safe to call more than once: existing worlds are re-attached rather than
    /// duplicated, and a failure for one stratum does not stop the others.
    ///
    /// Returns the number of worlds that are available after the call.
    pub fn setup(context: &Context) -> usize {
        let server = context.get_server();

        for depth in Depth::ALL {
            let name = depth.world_name();
            let _world = server.create_world(&name, Dimension::Overworld);

            if server.has_world(&name) {
                info!("World '{name}' ready");
                READY.with(|r| r.borrow_mut().insert(depth));
            } else {
                warn!("World '{name}' was not created");
            }
        }

        let count = READY.with(|r| r.borrow().len());
        info!("World manager: {count} layer world(s) available");
        count
    }

    /// Returns the world for `depth`, if [`WorldManager::setup`] made it available.
    ///
    /// The handle is resolved from the server on each call, so the returned world
    /// reflects the world's current state.
    #[must_use]
    pub fn world(context: &Context, depth: Depth) -> Option<World> {
        if !Self::is_ready(depth) {
            return None;
        }
        context.get_server().get_world_by_name(&depth.world_name())
    }

    /// Returns whether `depth`'s world was made available by [`WorldManager::setup`].
    #[must_use]
    pub fn is_ready(depth: Depth) -> bool {
        READY.with(|r| r.borrow().contains(&depth))
    }

    /// Returns the number of worlds currently tracked.
    #[must_use]
    pub fn count() -> usize {
        READY.with(|r| r.borrow().len())
    }
}
