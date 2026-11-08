//! Detects a player falling into a layer's void.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::player_move::PlayerMoveEvent;
use pumpkin_plugin_api::events_wit::PlayerMoveEventData;

/// Y coordinate below which a player is considered to have fallen into the void.
///
/// The world floor is -64, so this is a few dozen blocks below it: enough time to
/// react before the player takes void damage, but shallow enough to trigger promptly.
pub const VOID_Y: f64 = -128.0;

/// Detects a player leaving a layer by falling into its void.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerLeave;

impl EventHandler<PlayerMoveEvent> for LayerLeave {
    fn handle(&self, _server: Server, event: PlayerMoveEventData) -> PlayerMoveEventData {
        let world_name = event.player.get_world().get_name();
        let Some(layer) = crate::layer_for_world(&world_name) else {
            return event;
        };

        if event.to_position.1 < VOID_Y {
            // TODO: send the player to the next layer down.
            let _ = layer;
        }

        event
    }
}
