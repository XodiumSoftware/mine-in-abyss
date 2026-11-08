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
//!
//! ## Configuration
//!
//! | Field            | Default     | Description                                          |
//! | ---------------- | ----------- | ---------------------------------------------------- |
//! | `enabled`        | `false`     | Master toggle for per-layer worlds                   |
//! | `dimension`      | `overworld` | Minecraft dimension the layer worlds are created in  |
//! | `strata`         | all on      | Which strata get a world, keyed by stratum name      |

use crate::config::ConfigManager;
use crate::layers::layer::Depth;
use pumpkin_plugin_api::{Context, Dimension, World};
use serde::{Deserialize, Serialize};
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
    /// Creates a world for every enabled stratum and remembers it.
    ///
    /// Safe to call more than once: existing worlds are re-attached rather than
    /// duplicated, and a failure for one stratum does not stop the others.
    ///
    /// Returns the number of worlds that are available after the call.
    pub fn setup(context: &Context) -> usize {
        let Some(config) = ConfigManager::get() else {
            warn!("World manager skipped: config is not loaded");
            return 0;
        };

        let settings = &config.layers.worlds;
        if !settings.enabled {
            return 0;
        }

        let dimension = settings.dimension.to_dimension();
        let server = context.get_server();

        for depth in Depth::ALL {
            if !settings.is_stratum_enabled(depth) {
                continue;
            }

            let name = depth.world_name();
            let _world = server.create_world(name, dimension);

            if server.has_world(name) {
                info!("World '{name}' ready for {}", depth.name());
                READY.with(|r| r.borrow_mut().insert(depth));
            } else {
                warn!("World '{name}' was not created for {}", depth.name());
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
        context.get_server().get_world_by_name(depth.world_name())
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

/// Configuration for per-layer worlds.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WorldsConfig {
    /// Whether a world is created for each enabled stratum.
    pub enabled: bool,
    /// The Minecraft dimension layer worlds are created in.
    pub dimension: DimensionChoice,
    /// Per-stratum toggles, keyed by [`Depth`] variant name in `snake_case`.
    pub strata: StrataToggles,
}

impl WorldsConfig {
    /// Returns whether the given stratum should get its own world.
    #[must_use]
    pub fn is_stratum_enabled(&self, depth: Depth) -> bool {
        self.strata.is_enabled(depth)
    }
}

/// Which Minecraft dimension layer worlds are created in.
///
/// Serialized in `snake_case` (`overworld`, `nether`, `end`) in `config.toml`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionChoice {
    /// The overworld dimension.
    #[default]
    Overworld,
    /// The nether dimension.
    Nether,
    /// The end dimension.
    End,
}

impl DimensionChoice {
    /// Converts this choice into the plugin API's [`Dimension`].
    #[must_use]
    pub const fn to_dimension(self) -> Dimension {
        match self {
            Self::Overworld => Dimension::Overworld,
            Self::Nether => Dimension::Nether,
            Self::End => Dimension::End,
        }
    }
}

/// Per-stratum world toggles.
///
/// Fields are named after the [`Depth`] variants; each defaults to `true` so that
/// enabling [`WorldsConfig::enabled`] creates a world for every stratum.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct StrataToggles {
    /// Whether the First Layer gets a world.
    pub first: bool,
    /// Whether the Second Layer gets a world.
    pub second: bool,
    /// Whether the Third Layer gets a world.
    pub third: bool,
    /// Whether the Fourth Layer gets a world.
    pub fourth: bool,
    /// Whether the Fifth Layer gets a world.
    pub fifth: bool,
    /// Whether the Sixth Layer gets a world.
    pub sixth: bool,
    /// Whether the Seventh Layer gets a world.
    pub seventh: bool,
}

impl Default for StrataToggles {
    fn default() -> Self {
        Self {
            first: true,
            second: true,
            third: true,
            fourth: true,
            fifth: true,
            sixth: true,
            seventh: true,
        }
    }
}

impl StrataToggles {
    /// Returns the toggle for `depth`.
    #[must_use]
    pub const fn is_enabled(&self, depth: Depth) -> bool {
        match depth {
            Depth::First => self.first,
            Depth::Second => self.second,
            Depth::Third => self.third,
            Depth::Fourth => self.fourth,
            Depth::Fifth => self.fifth,
            Depth::Sixth => self.sixth,
            Depth::Seventh => self.seventh,
        }
    }
}
