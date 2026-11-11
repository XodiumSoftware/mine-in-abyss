//! The one-shot latch that keeps a layer transition from repeating.

use std::cell::RefCell;
use std::collections::HashSet;

use pumpkin_plugin_api::Player;

use super::general::{exempt_gamemode, is_dead, key};

thread_local! {
    /// Players currently being moved between layers, keyed by player id.
    ///
    /// `PlayerMoveEvent` fires on every movement, so a single transition would
    /// otherwise re-trigger on each tick the player spends inside the void or ceiling
    /// band — teleporting them repeatedly and adding curse strain each time. This
    /// latch makes a transition one-shot.
    ///
    /// It is held from the moment the teleport is issued until the player arrives in the
    /// destination world (see [`end_transition`]), not just for the remainder of the
    /// handler: the world change and the moves around it are not guaranteed to finish
    /// inside one event, so releasing immediately would let the tail of the transition
    /// re-trigger in the new world.
    ///
    /// Deliberately **not** persistent: it only describes an in-flight transition, and
    /// a stale entry after a restart would lock a player out of transitions forever.
    static TRANSITING: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Marks `player` as transitioning, returning `false` if they already were.
///
/// Use the returned value as a guard: only the caller that flipped the latch runs the
/// transition, and it must release it with [`end_transition`].
#[must_use]
pub fn begin_transition(player: &Player) -> bool {
    TRANSITING.with(|set| set.borrow_mut().insert(key(player)))
}

/// Releases `player`'s latch, returning `true` if they were latched.
///
/// Called from two places, and both are required for correctness:
///
/// - `LayerLeave` releases a latch it took but could not act on (no destination world,
///   or a terminal layer with nowhere to go), so a failed attempt never blocks the next
///   one.
/// - `LayerJoin` releases it on `PlayerChangedWorldEvent`, which is what actually ends a
///   successful transition. Releasing here rather than inside the move handler means the
///   moves that follow the teleport are still suppressed until the player is really in
///   the new world.
pub fn end_transition(player: &Player) {
    TRANSITING.with(|set| set.borrow_mut().remove(&key(player)));
}

/// Returns `true` if `player` is in a state where a layer transition must not run.
///
/// Players in **creative** or **spectator** mode move freely: they are never teleported
/// between layers and never afflicted by the curse, so builders and onlookers can pass
/// through the void and ceiling without being pulled around.
///
/// **Dead** players are skipped too: the curse can kill inside the move handler, and the
/// move events that follow a death must not teleport a corpse or re-apply the curse.
#[must_use]
pub fn ignores_transitions(player: &Player) -> bool {
    exempt_gamemode(player) || is_dead(player)
}
