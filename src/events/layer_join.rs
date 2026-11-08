//! Handles a player joining a layer world.

use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events_wit::PlayerChangedWorldEventData;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Player, Server};

use crate::layers::layer::Layer;

/// Handles a player joining a layer's world.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerJoin;

impl EventHandler<PlayerChangedWorldEvent> for LayerJoin {
    fn handle(
        &self,
        _server: Server,
        event: PlayerChangedWorldEventData,
    ) -> PlayerChangedWorldEventData {
        let world_name = event.to_world.get_name();
        if let Some(layer) = crate::layer_for_world(&world_name)
            && layer.enabled()
        {
            show_entry_title(layer, &event.player);
        }
        event
    }
}

/// Title fade-in time in ticks (half a second).
const TITLE_FADE_IN: i32 = 10;
/// How long the title stays on screen in ticks (two seconds).
const TITLE_STAY: i32 = 40;
/// Title fade-out time in ticks (half a second).
const TITLE_FADE_OUT: i32 = 10;

/// Shows a layer's name as a gradient title, with an "Entered …" subtitle.
fn show_entry_title(layer: &dyn Layer, player: &Player) {
    let colors: [RgbColor; 2] = layer.color();
    let title = TextComponent::text(layer.name()).gradient(&colors[..]);

    player.show_title(title);
    player.show_subtitle(TextComponent::text(&format!("Entered {}", layer.name())));
    player.send_title_animation(TITLE_FADE_IN, TITLE_STAY, TITLE_FADE_OUT);
}
