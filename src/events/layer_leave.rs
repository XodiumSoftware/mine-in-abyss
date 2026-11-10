//! Detects a player leaving a layer through its void or ceiling.
//!
//! Players in **creative or spectator mode** are left alone: they fly freely and can
//! pass through the void and ceiling without being teleported between layers or
//! afflicted by the curse.

use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;
use pumpkin_plugin_api::{Player, Server};

use crate::section::Section;
use crate::utils::ignores_transitions;

/// Detects a player leaving a layer through its void or ceiling.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerLeave;

impl EventHandler<PlayerMoveEvent> for LayerLeave {
    fn handle(&self, server: Server, event: PlayerMoveEventData) -> PlayerMoveEventData {
        if ignores_transitions(&event.player) {
            return event;
        }

        let world_name = event.player.get_world().get_name();
        let Some(layer) = crate::layer_for_world(&world_name) else {
            return event;
        };

        match event.to_position.1 {
            y if y < Section::VOID_Y => down(&server, &event.player, layer.id().next()),
            y if y > Section::MAX_Y => up(&server, &event.player, layer.id().prev()),
            _ => {}
        }

        event
    }
}

/// Sends a player up to `section`, keeping their X and Z position.
fn up(server: &Server, player: &Player, section: Option<Section>) {
    let Some(section) = section else {
        return;
    };
    let Some(world) = server.get_world_by_name(&section.world()) else {
        return;
    };

    super::curse::apply(player, section);

    let (x, _, z) = player.get_position();
    player.teleport((x, section.exit_y(), z), None, None, world);
}

/// Sends a player down to `section`, keeping their X and Z position.
///
/// If there is no section below — the player fell out of the bottom of the Abyss —
/// nothing happens: the void below the world floor kills them, as it would anywhere else.
/// Killing them here would instead fire on every move event while they are still in the
/// void band.
fn down(server: &Server, player: &Player, section: Option<Section>) {
    let Some(section) = section else {
        return;
    };
    let Some(world) = server.get_world_by_name(&section.world()) else {
        return;
    };

    let (x, _, z) = player.get_position();
    player.teleport((x, section.entry_y(), z), None, None, world);
}
