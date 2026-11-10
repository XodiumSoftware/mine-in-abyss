//! Records teleports so they are not mistaken for climbing.
//!
//! A teleport moves a player without them having climbed or fallen the Abyss, so it must
//! not be afflicted by the curse, and any descent it interrupts must not be resumed.
//! `PlayerTeleportEvent` has no world field, so this handler only notes where the player
//! landed; [`LayerLeave`](super::layer_leave) decides whether that landing point counts as
//! an ascent. The fall carry is dropped here because the drop it described is over.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_teleport::PlayerTeleportEvent;
use pumpkin_plugin_api::events_wit::PlayerTeleportEventData;

use crate::utils::{clear_fall, note_teleport};

/// Notes the destination of every player teleport.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerTeleport;

impl EventHandler<PlayerTeleportEvent> for LayerTeleport {
    fn handle(&self, _server: Server, event: PlayerTeleportEventData) -> PlayerTeleportEventData {
        note_teleport(&event.player, event.to_position.1);
        clear_fall(&event.player);
        event
    }
}
