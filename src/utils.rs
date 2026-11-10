//! Small helpers shared across the plugin's events and layers.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::GameMode;

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
