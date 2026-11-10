//! Detects a player leaving a layer through its void or ceiling.
//!
//! Players in **creative or spectator mode** are left alone: they fly freely and can
//! pass through the void and ceiling without being teleported between layers or
//! afflicted by the curse.
//!
//! A transition is **one-shot**: the move event fires on every movement, so a latch
//! prevents a player standing inside the void or ceiling band from being teleported (and
//! cursed) again and again.
//!
//! A successful transition also **cancels** the move event: the teleport has already
//! placed the player, and the original move describes a position inside the band of the
//! world they just left.
//!
//! If the destination world is unavailable — because
//! [`Worlds::setup`](crate::Worlds::setup) failed for that layer — the failure is logged
//! rather than swallowed, so the player being stuck in the void leaves a trace.

use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;
use pumpkin_plugin_api::{Player, Server};
use tracing::warn;

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

        let moved = match direction {
            Direction::Down => {
                // Below `Seventh` there is nowhere to go: vanilla void damage kills them.
                layer
                    .id()
                    .next()
                    .is_some_and(|section| down(&server, &event.player, section))
            }
            Direction::Up => layer
                .id()
                .prev()
                .is_some_and(|section| up(&server, &event.player, section)),
        };
        end_transition(&event.player);

        if moved {
            // The teleport already put the player where they belong, so the original
            // move must not also be applied: it describes a position inside the void or
            // ceiling band of the world they just left.
            PlayerMoveEventData {
                cancelled: true,
                ..event
            }
        } else {
            event
        }
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
///
/// Returns `false` if the destination world is unavailable, leaving the event untouched.
fn up(server: &Server, player: &Player, section: Section) -> bool {
    let Some(world) = world_for(server, section) else {
        return false;
    };

    super::curse::apply(player, section);

    let (x, _, z) = player.get_position();
    player.teleport((x, section.exit_y(), z), None, None, world);
    true
}

/// Sends a player down to `section`, keeping their X and Z position.
///
/// Returns `false` if the destination world is unavailable, leaving the event untouched.
fn down(server: &Server, player: &Player, section: Section) -> bool {
    let Some(world) = world_for(server, section) else {
        return false;
    };

    let (x, _, z) = player.get_position();
    player.teleport((x, section.entry_y(), z), None, None, world);
    true
}

/// Resolves the world for `section`, logging when it cannot be found.
///
/// A missing world means [`Worlds::setup`](crate::Worlds::setup) failed for that layer;
/// without a log the player would simply stay in the void with no trace of why.
fn world_for(server: &Server, section: Section) -> Option<pumpkin_plugin_api::World> {
    let name = section.world();
    let world = server.get_world_by_name(&name);
    if world.is_none() {
        warn!(
            "World '{name}' is unavailable; {} cannot be entered",
            section.id()
        );
    }
    world
}
