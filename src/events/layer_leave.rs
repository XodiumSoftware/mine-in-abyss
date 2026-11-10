//! Detects a player leaving a layer through its void or ceiling.
//!
//! Players in **creative or spectator mode** are left alone, as are **dead** players: they
//! are never teleported between layers or afflicted by the curse.
//!
//! A transition is **one-shot**: the move event fires on every movement, so a latch
//! prevents a player standing inside the void or ceiling band from being teleported (and
//! cursed) again and again. The latch is taken here and released by
//! [`LayerJoin`](super::layer_join) once the world change lands, so only a transition that
//! starts but cannot finish releases it here.
//!
//! A successful transition also **cancels** the move event: the teleport has already
//! placed the player, and the original move describes a position inside the band of the
//! world they just left.
//!
//! If the destination world is unavailable — because
//! [`Worlds::setup`](crate::Worlds::setup) failed for that layer — the failure is logged
//! rather than swallowed, so the player being stuck in the void leaves a trace.
//!
//! Leaving the Abyss at either end — above `First` or below `Seventh` — tells the player
//! there is nowhere to go, rather than doing nothing.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Player, Server};
use tracing::warn;

use crate::section::Section;
use crate::utils::{begin_transition, consume_teleport, end_transition, ignores_transitions};

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

        if arrived_by_teleport(&event.player, event.to_position.1) {
            return event;
        }

        let direction = match event.to_position.1 {
            y if y < Section::VOID_Y => Direction::Down,
            y if y > Section::MAX_Y => Direction::Up,
            _ => return event,
        };

        if !begin_transition(&event.player) {
            return event;
        }

        let moved = match direction {
            Direction::Down => {
                if let Some(section) = layer.next() {
                    down(&server, &event.player, section)
                } else {
                    tell(&event.player, "There is nothing below the Abyss.");
                    false
                }
            }
            Direction::Up => {
                if let Some(section) = layer.prev() {
                    up(&server, &event.player, section)
                } else {
                    tell(&event.player, "There is nothing above the Abyss.");
                    false
                }
            }
        };

        if moved {
            // The latch stays taken: `LayerJoin` releases it once the player is actually in
            // the destination world, so the moves and the world change that follow the
            // teleport cannot re-trigger the transition.
            PlayerMoveEventData {
                cancelled: true,
                ..event
            }
        } else {
            end_transition(&event.player);
            event
        }
    }
}

/// Returns `true` if this move merely reports a position a teleport already placed the
/// player at.
///
/// Being teleported above the ceiling is not an ascent, and must not be cursed. The
/// record is consumed either way, so a genuine climb afterwards still counts; the Y
/// comparison stops a stale record swallowing an unrelated move.
fn arrived_by_teleport(player: &Player, y: f64) -> bool {
    consume_teleport(player)
        .is_some_and(|teleported_y| (teleported_y - y).abs() < TELEPORT_TOLERANCE)
}

/// How far a move may differ from a teleport destination and still count as the same
/// arrival, in blocks. Covers the drift between the requested and applied position.
const TELEPORT_TOLERANCE: f64 = 2.0;

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
    player.teleport((x, Section::exit_y(), z), None, None, world);
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
    player.teleport((x, Section::entry_y(), z), None, None, world);
    true
}

/// Tells `player` something went nowhere, as a grey system message.
///
/// Used when a transition is impossible rather than failing silently, so a player is not
/// left guessing why nothing happened.
fn tell(player: &Player, message: &str) {
    let text = TextComponent::text(message).color_named(NamedColor::Gray);
    player.send_system_message(text, false);
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
