//! Sends a respawning player back to the top of the Abyss.
//!
//! The Abyss is escaped on its own terms, so death is not a shortcut home: the player
//! wakes up at `First`'s world spawn rather than wherever they died. Since
//! [`BedSpawn`](super::bed_spawn::BedSpawn) refuses beds, there is no way to bank a spawn
//! point further down and skip the climb.
//!
//! # Why this event
//!
//! `PlayerRespawnEvent` is fired once the host has already resolved a destination world and
//! position, and it is **not cancellable** — so this handler does not prevent the respawn,
//! it moves the player again immediately afterwards. The host resolves `First`'s position
//! only if the player died in it; anywhere else it would have used the `OVERWORLD`
//! dimension's own spawn, which is not a layer world.
//!
//! Rewriting `PlayerSpawnLocationEvent` instead would move the position but not the world,
//! which is not enough: the player has to change world.
//!
//! # What it does not do
//!
//! Respawn still happens first — the player is alive and in the resolved world for an
//! instant — so this is a teleport, not a redirection. Nothing observes that in between
//! except the respawn packet itself, which is what puts the player back in a playable
//! state.

use pumpkin_plugin_api::events::player::player_respawn::PlayerRespawnEvent;
use pumpkin_plugin_api::events::{EventHandler, EventPriority};
use pumpkin_plugin_api::events_wit::PlayerRespawnEventData;
use pumpkin_plugin_api::{Context, Server};

use crate::events::event::Event;
use crate::layer::Layer;
use crate::utils::{clear_fall, world_for};

/// Returns a respawning player to `First`'s world spawn.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerRespawn;

impl Event for LayerRespawn {
    fn register(context: &Context) {
        context
            .register_event_handler::<PlayerRespawnEvent, _>(
                LayerRespawn,
                EventPriority::Normal,
                false,
            )
            .expect("failed to register PlayerRespawnEvent handler");
    }
}

impl EventHandler<PlayerRespawnEvent> for LayerRespawn {
    fn handle(&self, server: Server, event: PlayerRespawnEventData) -> PlayerRespawnEventData {
        super::curse::cure(&event.player);
        clear_fall(&event.player);

        let Some(world) = world_for(&server, Layer::First, "a respawn cannot return to First")
        else {
            return event;
        };

        let spawn = world.get_spawn_location();
        event.player.teleport(
            (
                f64::from(spawn.pos.x) + 0.5,
                f64::from(spawn.pos.y),
                f64::from(spawn.pos.z) + 0.5,
            ),
            Some(spawn.yaw),
            Some(spawn.pitch),
            world,
        );

        event
    }
}
