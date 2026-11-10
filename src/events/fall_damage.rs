//! Settles the fall distance a descent accumulates across layer boundaries.
//!
//! Vanilla resets a player's fall distance whenever they change world, and a layer world is
//! only 384 blocks tall. A drop that spans several layers would therefore be a series of
//! separate falls, each well inside the height of the layer it happened in — so a player
//! could fall from `First` to `Sixth` and be hurt by none of them.
//!
//! [`LayerLeave`](super::layer_leave) measures the distance at the moment of each swap and
//! [`carry_fall`](crate::utils::carry_fall) keeps it. The landing itself belongs to the
//! next layer, where it no longer counts as a layer transition: this handler adds the
//! carried distance to the fall the player is making there, so that when they reach ground
//! vanilla sees one fall of the whole drop.
//!
//! Damage is never applied here. The carried distance is handed back to vanilla, which
//! then computes the impact against whatever is actually under the player — water, hay
//! bales, a bed, slow falling and feather falling all keep working exactly as vanilla
//! defines them.
//!
//! A fall that leaves the layer again before landing is **not** joined up: the next
//! segment is recorded over the carry, so only the last layer of a drop is measured in
//! full. Joining them would put the damage somewhere the player is not, and vanilla's own
//! per-world damage stays in place as the baseline. The comment on
//! [`resume_fall`](crate::utils::resume_fall) records what closing that gap would cost.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;

use crate::utils::{ignores_transitions, resume_fall};

/// Adds the fall distance a descent carried into the layer a player is landing in.
#[derive(Debug, Default, Clone, Copy)]
pub struct FallDamage;

impl EventHandler<PlayerMoveEvent> for FallDamage {
    fn handle(&self, _server: Server, event: PlayerMoveEventData) -> PlayerMoveEventData {
        if ignores_transitions(&event.player) {
            return event;
        }

        let world_name = event.player.get_world().get_name();
        let Some(layer) = crate::layer_for_world(&world_name) else {
            return event;
        };

        if let Some(fallen) = resume_fall(&event.player, layer, event.to_position.1) {
            event.player.as_entity().set_fall_distance(fallen);
        }

        event
    }
}
