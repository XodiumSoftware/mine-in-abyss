//! `mine-in-abyss` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in
//! Rust that recreates ***Made in Abyss*** inside Minecraft.
//!
//! Each layer of the Abyss is its own Minecraft world, and players descend by falling
//! through the void of the layer above. Layers are [`Layer`] variants, which carry
//! their identifier, display name and palette; gameplay content for individual layers is
//! still a stub, but the Curse of the Abyss and layer transitions are implemented.
//!
//! See [`GUIDE.md`](../GUIDE.md) for installation.
#![allow(clippy::must_use_candidate)]

mod generator;
mod layer;
mod utils;
mod worlds;

/// The `/abyss` command.
mod commands {
    pub mod abyss;
}

/// Event handlers for `mine-in-abyss`.
mod events {
    pub mod curse;
    pub mod layer_join;
    pub mod layer_leave;
    pub mod layer_respawn;
    pub mod layer_teleport;
}

pub use layer::*;
pub use worlds::*;

use pumpkin_plugin_api::events::EventPriority;
use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events::player::player_respawn::PlayerRespawnEvent;
use pumpkin_plugin_api::events::player::player_teleport::PlayerTeleportEvent;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

/// The plugin identifier used for namespaced ids and permission nodes.
///
/// Must be a valid Minecraft identifier, so the crate name (`mine-in-abyss`)
/// is flattened to its underscore form.
pub const PLUGIN_ID: &str = "mine_in_abyss";

/// Every layer, from the surface down.
///
/// This is the single source of truth for which layers exist: registration, world
/// creation and the entry title all read from it. Identical to [`Layer::ALL`].
pub const LAYERS: [Layer; 7] = Layer::ALL;

/// Returns the layer whose world is called `name`, if any.
pub(crate) fn layer_for_world(name: &str) -> Option<Layer> {
    LAYERS.into_iter().find(|layer| layer.world() == name)
}

/// Entry point of the plugin.
pub struct MineInAbyss {}

impl MineInAbyss {
    /// Registers every layer, from the surface down.
    fn register_layers() {
        info!("Registered {} layer(s)", LAYERS.len());
    }

    /// Registers the event that announces a layer when a player enters its world.
    fn register_events(context: &Context) {
        context
            .register_event_handler::<PlayerChangedWorldEvent, _>(
                events::layer_join::LayerJoin,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerChangedWorldEvent handler");

        context
            .register_event_handler::<PlayerMoveEvent, _>(
                events::layer_leave::LayerLeave,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerMoveEvent handler");

        context
            .register_event_handler::<PlayerRespawnEvent, _>(
                events::layer_respawn::LayerRespawn,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerRespawnEvent handler");

        context
            .register_event_handler::<PlayerTeleportEvent, _>(
                events::layer_teleport::LayerTeleport,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerTeleportEvent handler");
    }
}

impl Plugin for MineInAbyss {
    fn new() -> Self {
        Self {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: PLUGIN_ID.into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS")
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            dependencies: vec![],
            permissions: vec![
                pumpkin_plugin_api::permissions::FS_READ_DATA.into(),
                pumpkin_plugin_api::permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        Self::register_layers();
        Self::register_events(&context);
        commands::abyss::register(&context);

        let worlds = Worlds::setup(&context, &LAYERS);
        info!("Mine in Abyss loaded ({worlds} layer world(s)).");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Mine in Abyss unloaded.");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(MineInAbyss);
