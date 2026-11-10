//! Small helpers shared across the plugin's events and layers.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::GameMode;
use pumpkin_plugin_api::wit::pumpkin::plugin::uuid;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    /// Players currently being moved between layers, keyed by player id.
    ///
    /// `PlayerMoveEvent` fires on every movement, so a single transition would
    /// otherwise re-trigger on each tick the player spends inside the void or ceiling
    /// band — teleporting them repeatedly and adding curse strain each time. This
    /// latch makes a transition one-shot.
    ///
    /// Deliberately **not** persistent: it only describes an in-flight transition, and
    /// a stale entry after a restart would lock a player out of transitions forever.
    static TRANSITING: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Returns a stable key for `player`, usable across separate WIT handles.
fn key(player: &Player) -> String {
    uuid::to_string(player.get_id())
}

/// Marks `player` as transitioning, returning `false` if they already were.
///
/// Use the returned value as a guard: only the caller that flipped the latch should
/// run the transition, and it must call [`end_transition`] to release it.
#[must_use]
pub fn begin_transition(player: &Player) -> bool {
    TRANSITING.with(|set| set.borrow_mut().insert(key(player)))
}

/// Clears `player`'s transition latch.
pub fn end_transition(player: &Player) {
    TRANSITING.with(|set| set.borrow_mut().remove(&key(player)));
}

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
