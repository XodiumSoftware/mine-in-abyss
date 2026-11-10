//! Restricts which mobs may spawn in each layer.
//!
//! Each layer lists the mobs it allows through `Layer::allowed_mobs`; anything else is
//! cancelled before it appears. This is the per-layer counterpart to the `spawn-monsters`
//! game rule, which can only switch spawning off wholesale.
//!
//! [`CreatureSpawnEvent`] carries the world and a spawn reason, so a spawn is checked
//! against the layer it is actually landing in. Spawns in non-Abyss worlds are left alone,
//! and spawner-driven spawns always pass, since a placed spawner is authored content.

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::entity::creature_spawn::CreatureSpawnEvent;
use pumpkin_plugin_api::events_wit::CreatureSpawnEventData;

/// Cancels spawns of mobs the layer does not allow.
#[derive(Debug, Default, Clone, Copy)]
pub struct LayerMobSpawn;

impl EventHandler<CreatureSpawnEvent> for LayerMobSpawn {
    fn handle(&self, _server: Server, event: CreatureSpawnEventData) -> CreatureSpawnEventData {
        if event.spawn_reason.eq_ignore_ascii_case(SPAWNER_REASON) {
            return event;
        }

        let world_name = event.target_world.get_name();
        let Some(layer) = crate::layer_for_world(&world_name) else {
            return event;
        };

        let Some(allowed) = layer.allowed_mobs() else {
            return event;
        };

        if is_allowed(allowed, &event.entity_type) {
            event
        } else {
            CreatureSpawnEventData {
                cancelled: true,
                ..event
            }
        }
    }
}

/// Spawn reason reported for mob spawners and trial spawners.
///
/// Deliberately placed spawners are authored content, not ambient spawning, so they bypass
/// the allowlist: a builder who puts a spawner in a layer means it.
const SPAWNER_REASON: &str = "spawner";

/// Returns `true` if `mob` is in `allowed`.
///
/// Minecraft reports entity types as namespaced ids, but accepts a bare name too, so both
/// forms are matched.
fn is_allowed(allowed: &[&str], mob: &str) -> bool {
    allowed
        .iter()
        .any(|candidate| *candidate == mob || candidate.strip_prefix("minecraft:") == Some(mob))
}
