//! Helpers about a player that more than one category needs.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::GameMode;
use pumpkin_plugin_api::wit::pumpkin::plugin::uuid;

/// Returns a stable key for `player`, usable across separate WIT handles.
///
/// The `thread_local` records key on this rather than on the `Player` itself: a handle
/// obtained from one event is not guaranteed to compare equal to one from another, but
/// the UUID always is.
pub fn key(player: &Player) -> String {
    uuid::to_string(player.get_id())
}

/// Returns `true` if `player` has died and has not respawned yet.
///
/// A player that is still dead keeps firing move events, and every handler that reacts to
/// one has to stop there: a corpse must not be teleported, cursed, or counted as landing
/// a fall.
#[must_use]
pub fn is_dead(player: &Player) -> bool {
    player
        .as_entity()
        .as_living()
        .is_some_and(|living| living.is_dead())
}

/// Returns `true` if `player` is in a gamemode that moves freely between layers.
///
/// Creative and spectator players are never teleported between layers and never
/// afflicted by the curse, so builders and onlookers can pass through the void and
/// ceiling without being pulled around.
#[must_use]
pub fn exempt_gamemode(player: &Player) -> bool {
    matches!(
        player.get_gamemode(),
        GameMode::Creative | GameMode::Spectator
    )
}
