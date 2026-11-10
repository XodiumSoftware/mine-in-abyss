//! Handles a player joining a layer world.
//!
//! This is also where a layer transition finishes: [`LayerLeave`](super::layer_leave) leaves
//! the player latched after teleporting them, and this handler releases that latch once
//! they are really in the new world.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events_wit::PlayerChangedWorldEventData;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::sounds::{Sound, SoundCategory};

use crate::layer::Layer;
use crate::utils::{end_transition, ordinal, rgb};

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
        if let Some(layer) = crate::layer_for_world(&world_name) {
            let subtitle = format!("Entered the {} layer", ordinal(layer.number()));
            let subtitle_colors: [RgbColor; 2] = [rgb(0xFF, 0xA7, 0x51), rgb(0xFF, 0xE2, 0x59)];

            event.player.show_title(
                TextComponent::text(layer.name())
                    .gradient(&layer.color())
                    .bold(true),
            );
            event
                .player
                .show_subtitle(TextComponent::text(&subtitle).gradient(&subtitle_colors));
            event.player.send_title_animation(20, 100, 30);
            event.player.play_sound(
                Sound::UiToastChallengeComplete,
                SoundCategory::Master,
                1.0,
                1.0,
            );
        }

        end_transition(&event.player);

        if world_name == Layer::First.world() {
            super::curse::cure(&event.player);
        }

        event
    }
}
