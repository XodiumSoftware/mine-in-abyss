//! Records teleports so they are not mistaken for climbing.
//!
//! A teleport moves a player without them having climbed the Abyss, so it must not be
//! afflicted by the curse. `PlayerTeleportEvent` has no world field, so this handler only
//! notes where the player landed; [`LayerLeave`](super::layer_leave) decides whether that
//! landing point counts as an ascent.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_teleport::PlayerTeleportEvent;
use pumpkin_plugin_api::events_wit::PlayerTeleportEventData;

use crate::utils::note_teleport;

/// Notes the destination of every player teleport.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerTeleport;

impl EventHandler<PlayerTeleportEvent> for LayerTeleport {
    fn handle(&self, _server: Server, event: PlayerTeleportEventData) -> PlayerTeleportEventData {
        note_teleport(&event.player, event.to_position.1);
        event
    }
}
