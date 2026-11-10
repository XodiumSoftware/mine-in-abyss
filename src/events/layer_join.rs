//! Handles a player joining a layer world.
//!
//! This is also where a layer transition finishes: [`LayerLeave`](super::layer_leave) leaves
//! the player latched after teleporting them, and this handler releases that latch once
//! they are really in the new world.

use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_changed_world::PlayerChangedWorldEvent;
use pumpkin_plugin_api::events_wit::PlayerChangedWorldEventData;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Player, Server};

use crate::section::Section;
use crate::utils::end_transition;

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
        if let Some(section) = crate::layer_for_world(&world_name) {
            show_entry_title(section, &event.player);
        }

        end_transition(&event.player);

        if world_name == Section::First.world() {
            super::curse::cure(&event.player);
        }

        event
    }
}

/// Shows a section's name as a gradient title, with an "Entered …" subtitle.
fn show_entry_title(section: Section, player: &Player) {
    let colors: [RgbColor; 2] = section.color();
    let title = TextComponent::text(section.name()).gradient(&colors[..]);

    player.show_title(title);
    player.show_subtitle(TextComponent::text(&format!("Entered {}", section.name())));
    player.send_title_animation(10, 40, 10);
}
