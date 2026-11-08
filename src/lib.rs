//! `mine-in-abyss` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in
//! Rust that recreates ***Made in Abyss*** inside Minecraft.
//!
//! This crate is currently a **stub**: the plugin boots, loads its config, and exposes the layer
//! registration pipeline that future content (the Abyss layers, relics, curses, delvers, ...) will
//! plug into.
//!
//! See [`GUIDE.md`](../GUIDE.md) for installation and configuration.
#![allow(clippy::must_use_candidate)]

mod config;
mod layers {
    pub mod fifth;
    pub mod first;
    pub mod fourth;
    pub mod layer;
    pub mod second;
    pub mod seventh;
    pub mod sixth;
    pub mod third;
}
mod world_manager;

pub use config::*;
pub use layers::*;
pub use world_manager::*;

use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events::{EventHandler, EventPriority};
use pumpkin_plugin_api::events_wit::PlayerChangedWorldEventData;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, Server};
use tracing::info;

use crate::layers::fifth::Fifth;
use crate::layers::first::First;
use crate::layers::fourth::Fourth;
use crate::layers::layer::{Depth, Layer};
use crate::layers::second::Second;
use crate::layers::seventh::Seventh;
use crate::layers::sixth::Sixth;
use crate::layers::third::Third;

/// Announces a layer's name when a player enters its world.
#[derive(Debug, Default, Clone, Copy)]
struct LayerEntryTitle;

impl EventHandler<PlayerChangedWorldEvent> for LayerEntryTitle {
    fn handle(
        &self,
        _server: Server,
        event: PlayerChangedWorldEventData,
    ) -> PlayerChangedWorldEventData {
        let world_name = event.to_world.get_name();
        if let Some(depth) = Depth::from_world_name(&world_name) {
            let layer: &dyn Layer = match depth {
                Depth::First => &First,
                Depth::Second => &Second,
                Depth::Third => &Third,
                Depth::Fourth => &Fourth,
                Depth::Fifth => &Fifth,
                Depth::Sixth => &Sixth,
                Depth::Seventh => &Seventh,
            };
            if layer.enabled() {
                depth.announce_entry(&event.player);
            }
        }
        event
    }
}

/// The plugin identifier used for namespaced ids and permission nodes.
///
/// Must be a valid Minecraft identifier, so the crate name (`mine-in-abyss`)
/// is flattened to its underscore form.
pub const PLUGIN_ID: &str = "mine_in_abyss";

/// Entry point of the plugin.
pub struct MineInAbyss {}

impl MineInAbyss {
    /// Registers every stratum's layer, from the surface down.
    fn register_layers(context: &Context) {
        let layers: Vec<&dyn Layer> =
            vec![&First, &Second, &Third, &Fourth, &Fifth, &Sixth, &Seventh];

        let enabled = layers.iter().filter(|l| l.enabled()).count();
        for layer in layers {
            layer.register(context);
        }

        info!("Registered {enabled} layer(s)");
    }

    /// Registers the event that announces a layer when a player enters its world.
    fn register_events(context: &Context) {
        context
            .register_event_handler::<PlayerChangedWorldEvent, _>(
                LayerEntryTitle,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerChangedWorldEvent handler");
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
        ConfigManager::load(&context);

        Self::register_layers(&context);
        Self::register_events(&context);

        let worlds = WorldManager::setup(&context);
        info!("Mine in Abyss loaded ({worlds} layer world(s)).");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Mine in Abyss unloaded.");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(MineInAbyss);
