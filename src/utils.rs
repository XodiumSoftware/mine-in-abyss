//! Small helpers shared across the plugin's events and layers.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::GameMode;

/// Y coordinate below which a player is considered to have fallen into the void.
///
/// The world floor is -64, so this is a few dozen blocks below it: enough time to
/// react before the player takes void damage, but shallow enough to trigger promptly.
pub const VOID_Y: f64 = -128.0;

/// Y coordinate above which a player is considered to have left through the ceiling.
///
/// The build limit is 319, so this is just above it.
pub const MAX_Y: f64 = 320.0;

/// Returns `true` if `player` is exempt from layer transitions.
///
/// Players in **creative** or **spectator** mode move freely: they are never
/// teleported between layers and never afflicted by the curse, so builders and
/// onlookers can pass through the void and ceiling without being pulled around.
#[must_use]
pub fn ignores_transitions(player: &Player) -> bool {
    matches!(
        player.get_gamemode(),
        GameMode::Creative | GameMode::Spectator
    )
}
