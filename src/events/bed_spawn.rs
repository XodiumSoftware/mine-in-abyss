//! Refuses the spawn points a player tries to set for themselves.
//!
//! The Abyss is meant to be descended and escaped on its own terms: a bed at the bottom of
//! `Seventh` would let a player skip the whole climb — and the Curse that comes with it —
//! by dying there. Setting a spawn point is therefore refused, so dying sends the player
//! back to the world spawn instead.
//!
//! # What is refused
//!
//! Only a spawn point the player sets **by using a block**, which the host marks by calling
//! `set_respawn_point` with `forced: false`. That covers beds and respawn anchors, and it
//! deliberately excludes `/spawnpoint`, which passes `forced: true` because an operator
//! asked for it. Operators keep their escape hatch; players do not get a free one.
//!
//! Clearing a spawn point (`new_spawn: None`) is left alone, so anything that removes a
//! respawn point still works.
//!
//! # What the player sees
//!
//! Cancelling the event is enough: the host never stores the point, and never sends its
//! usual "Spawn point set" message, so this handler adds a short red action bar line —
//! using a bed should not silently do nothing.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_spawn_change::PlayerSpawnChangeEvent;
use pumpkin_plugin_api::events_wit::PlayerSpawnChangeEventData;
use pumpkin_plugin_api::{Context, Server};

use crate::events::event::Event;
use crate::utils::tell;

/// Refuses a player-set spawn point, leaving cleared and operator-set points alone.
#[derive(Debug, Default, Clone, Copy)]
pub struct BedSpawn;

impl Event for BedSpawn {
    fn register(context: &Context) {
        context
            .register_event_handler::<PlayerSpawnChangeEvent, _>(
                BedSpawn,
                Self::PRIORITY,
                Self::BLOCKING,
            )
            .unwrap_or_else(|_| Self::fail("BedSpawn"));
    }
}

impl EventHandler<PlayerSpawnChangeEvent> for BedSpawn {
    fn handle(
        &self,
        _server: Server,
        event: PlayerSpawnChangeEventData,
    ) -> PlayerSpawnChangeEventData {
        if event.forced || event.new_spawn.is_none() {
            return event;
        }

        tell(
            &event.player,
            "The Abyss does not let you set a spawn point.",
            NamedColor::Red,
        );

        PlayerSpawnChangeEventData {
            cancelled: true,
            ..event
        }
    }
}
