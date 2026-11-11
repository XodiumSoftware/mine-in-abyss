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
//! A descent **carries its fall distance** into the next layer. Vanilla resets the
//! distance when the world changes, so the descent would otherwise end the fall: the
//! player would fall out of `First` unhurt, and only the last layer would hurt them. The
//! distance is measured at the moment of the swap and handed to
//! [`FallDamage`](super::fall_damage), which adds it to the fall in the layer arrived in.
//!
//! A descent also **carries the monsters**. Entities cannot change layers, so the mobs a
//! player leaves behind in an empty layer above would simply stop existing the moment the
//! player falls out. Descending therefore moves the players in the layer above down with
//! them, at their own X/Z and the next entry height, before the player is placed. The
//! mobs that were chasing them follow; the rest of the layer, monsters that had not yet
//! found them included, does not. The monsters are scaled by
//! [`Layer::monster_scale`] on arrival, so what follows is as dangerous as the layer it
//! has followed the player into.
//!
//! Leaving the Abyss at either end — above `First` or below `Seventh` — tells the player
//! there is nowhere to go, rather than doing nothing. Below `Seventh` the carry is
//! dropped instead: there is no layer for it to be resumed into, and vanilla's void
//! damage is the death the drop already deserves.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;
use pumpkin_plugin_api::{Context, Player, Server};

use crate::events::event::Event;
use crate::layer::Layer;
use crate::utils::{
    arrived_by_teleport, begin_transition, carry_fall, clear_fall, ignores_transitions, layer_of,
    tell, world_for,
};
use crate::world::setup::Worlds;

/// Detects a player leaving a layer through its void or ceiling.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerLeave;

impl Event for LayerLeave {
    fn register(context: &Context) {
        context
            .register_event_handler::<PlayerMoveEvent, _>(
                LayerLeave,
                Self::PRIORITY,
                Self::BLOCKING,
            )
            .unwrap_or_else(|_| Self::fail("LayerLeave"));
    }
}

impl EventHandler<PlayerMoveEvent> for LayerLeave {
    fn handle(&self, server: Server, event: PlayerMoveEventData) -> PlayerMoveEventData {
        if ignores_transitions(&event.player) {
            return event;
        }

        let Some(layer) = layer_of(&event.player) else {
            return event;
        };

        if arrived_by_teleport(&event.player, event.to_position.1) {
            return event;
        }

        let direction = match event.to_position.1 {
            y if y < Layer::MIN_Y => Direction::Down,
            y if y > Layer::MAX_Y => Direction::Up,
            _ => return event,
        };

        if !begin_transition(&event.player) {
            return event;
        }

        let moved = match direction {
            Direction::Down => {
                if let Some(layer) = layer.next() {
                    down(&server, &event.player, layer)
                } else {
                    tell(
                        &event.player,
                        "There is nothing below the Abyss.",
                        NamedColor::Gray,
                    );
                    clear_fall(&event.player);
                    false
                }
            }
            Direction::Up => {
                if let Some(layer) = layer.prev() {
                    up(&server, &event.player, layer)
                } else {
                    tell(
                        &event.player,
                        "There is nothing above the Abyss.",
                        NamedColor::Gray,
                    );
                    clear_fall(&event.player);
                    false
                }
            }
        };

        if !moved {
            return event;
        }

        PlayerMoveEventData {
            cancelled: true,
            ..event
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

/// Sends a player up to `layer`, keeping their X and Z position.
///
/// Returns `false` if the destination world is unavailable, leaving the event untouched.
fn up(server: &Server, player: &Player, layer: Layer) -> bool {
    let Some(world) = world_for(server, layer, "it cannot be entered") else {
        return false;
    };

    super::curse::apply(player, layer);

    let (x, _, z) = player.get_position();
    player.teleport((x, Layer::exit_y(), z), None, None, world);
    true
}

/// Sends a player down to `layer`, keeping their X and Z position.
///
/// Returns `false` if the destination world is unavailable, leaving the event untouched.
fn down(server: &Server, player: &Player, layer: Layer) -> bool {
    let Some(world) = world_for(server, layer, "it cannot be entered") else {
        return false;
    };

    let fallen_before_world_change = player.as_entity().get_fall_distance();

    Worlds::descend_monsters(server, &player.get_world(), &world, layer);

    let (x, _, z) = player.get_position();
    player.teleport((x, Layer::entry_y(), z), None, None, world);

    carry_fall(player, fallen_before_world_change, layer);
    true
}
