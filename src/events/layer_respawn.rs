//! Handles a player respawning.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_respawn::PlayerRespawnEvent;
use pumpkin_plugin_api::events_wit::PlayerRespawnEventData;

use crate::utils::clear_fall;

/// Clears the player's curse strain when they respawn.
///
/// The curse is uncurable by normal means, but death resets it: otherwise a fatal
/// strain would follow the player through every respawn. A descent they were partway
/// through dies with them, so the carried fall distance is dropped too.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerRespawn;

impl EventHandler<PlayerRespawnEvent> for LayerRespawn {
    fn handle(&self, _server: Server, event: PlayerRespawnEventData) -> PlayerRespawnEventData {
        super::curse::cure(&event.player);
        clear_fall(&event.player);
        event
    }
}
