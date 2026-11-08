//! `mine-in-abyss` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in
//! Rust that recreates ***Made in Abyss*** inside Minecraft.
//!
//! This crate is currently a **stub**: the plugin boots and exposes the layer
//! registration pipeline that future content (the Abyss layers, relics, curses, delvers, ...) will
//! plug into.
//!
//! See [`GUIDE.md`](../GUIDE.md) for installation.
#![allow(clippy::must_use_candidate)]

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
mod section;
mod worlds;

pub use layers::*;
pub use section::*;
pub use worlds::*;

use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events::{EventHandler, EventPriority};
use pumpkin_plugin_api::events_wit::PlayerChangedWorldEventData;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, Server};
use tracing::info;

use crate::layers::fifth::Fifth;
use crate::layers::first::First;
use crate::layers::fourth::Fourth;
use crate::layers::layer::Layer;
use crate::layers::second::Second;
use crate::layers::seventh::Seventh;
use crate::layers::sixth::Sixth;
use crate::layers::third::Third;

/// The plugin identifier used for namespaced ids and permission nodes.
///
/// Must be a valid Minecraft identifier, so the crate name (`mine-in-abyss`)
/// is flattened to its underscore form.
pub const PLUGIN_ID: &str = "mine_in_abyss";

/// Title fade-in time in ticks (half a second).
pub const TITLE_FADE_IN: i32 = 10;
/// How long the title stays on screen in ticks (two seconds).
pub const TITLE_STAY: i32 = 40;
/// Title fade-out time in ticks (half a second).
pub const TITLE_FADE_OUT: i32 = 10;

/// Every layer, from the surface down.
///
/// This is the single source of truth for which layers exist: registration, world
/// creation and the entry title all read from it.
pub const LAYERS: [&dyn Layer; 7] = [&First, &Second, &Third, &Fourth, &Fifth, &Sixth, &Seventh];

/// Returns the layer whose world is called `name`, if any.
fn layer_for_world(name: &str) -> Option<&'static dyn Layer> {
    LAYERS.into_iter().find(|layer| layer.id().world() == name)
}

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
        if let Some(layer) = layer_for_world(&world_name)
            && layer.enabled()
        {
            show_entry_title(layer, &event.player);
        }
        event
    }
}

/// Shows a layer's name as a gradient title, with an "Entered …" subtitle.
fn show_entry_title(layer: &dyn Layer, player: &pumpkin_plugin_api::Player) {
    let colors: [RgbColor; 2] = layer.color();
    let title = TextComponent::text(layer.name()).gradient(&colors[..]);

    player.show_title(title);
    player.show_subtitle(TextComponent::text(&format!("Entered {}", layer.name())));
    player.send_title_animation(TITLE_FADE_IN, TITLE_STAY, TITLE_FADE_OUT);
}

/// Entry point of the plugin.
pub struct MineInAbyss {}

impl MineInAbyss {
    /// Registers every layer, from the surface down.
    fn register_layers() {
        let enabled = LAYERS.iter().filter(|layer| layer.enabled()).count();
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
        Self::register_layers();
        Self::register_events(&context);

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
