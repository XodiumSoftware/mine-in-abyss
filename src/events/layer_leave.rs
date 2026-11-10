//! Detects a player leaving a layer through its void or ceiling.
//!
//! Players in **creative or spectator mode** are left alone: they fly freely and can
//! pass through the void and ceiling without being teleported between layers or
//! afflicted by the curse.
//!
//! A transition is **one-shot**: the move event fires on every movement, so a latch
//! prevents a player standing inside the void or ceiling band from being teleported (and
//! cursed) again and again.

use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;
use pumpkin_plugin_api::{Player, Server};

use crate::section::Section;
use crate::utils::{begin_transition, end_transition, ignores_transitions};

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

        let direction = match event.to_position.1 {
            y if y < Section::VOID_Y => Direction::Down,
            y if y > Section::MAX_Y => Direction::Up,
            _ => return event,
        };

        // Take the latch only once a transition is actually about to happen, and release
        // it whatever the outcome, so a failed move can never lock the player out of
        // later transitions.
        if !begin_transition(&event.player) {
            return event;
        }

        match direction {
            Direction::Down => {
                if let Some(section) = layer.id().next() {
                    down(&server, &event.player, section);
                }
                // Below `Seventh` there is nowhere to go: vanilla void damage kills them.
            }
            Direction::Up => {
                if let Some(section) = layer.id().prev() {
                    up(&server, &event.player, section);
                }
            }
        }
        end_transition(&event.player);

        event
    }
}

/// Which way out of a layer a player is heading.
#[derive(Debug, Clone, Copy)]
enum Direction {
    /// Fell out of the bottom of the layer.
    Down,
    /// Rose through the ceiling of the layer.
    Up,
}

/// Sends a player up to `section`, keeping their X and Z position.
fn up(server: &Server, player: &Player, section: Section) {
    let Some(world) = server.get_world_by_name(&section.world()) else {
        return;
    };

    super::curse::apply(player, section);

    let (x, _, z) = player.get_position();
    player.teleport((x, section.exit_y(), z), None, None, world);
}

/// Sends a player down to `section`, keeping their X and Z position.
fn down(server: &Server, player: &Player, section: Section) {
    let Some(world) = server.get_world_by_name(&section.world()) else {
        return;
    };

    let (x, _, z) = player.get_position();
    player.teleport((x, section.entry_y(), z), None, None, world);
}
