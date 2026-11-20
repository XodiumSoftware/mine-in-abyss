//! The one-shot latch that keeps a layer transition from repeating.

use std::cell::RefCell;
use std::collections::HashSet;

use pumpkin_plugin_api::Player;

use super::general::{exempt_gamemode, is_dead, key};

thread_local! {
    /// Players currently being moved between layers, keyed by player id.
    ///
    /// Held until the player arrives in the destination world, not just for the remainder of
    /// the handler: the moves and world change around a teleport do not finish inside one
    /// event, so releasing sooner would let the tail re-trigger. Deliberately not persistent,
    /// so a stale entry cannot survive a restart.
    static TRANSITING: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Marks `player` as transitioning, returning `false` if they already were.
///
/// Only the caller that flipped the latch runs the transition, and it must release it with
/// [`end_transition`].
#[must_use]
pub(crate) fn begin_transition(player: &Player) -> bool {
    TRANSITING.with(|set| set.borrow_mut().insert(key(player)))
}

/// Releases `player`'s latch. Called by `LayerLeave` for an attempt it could not finish, and
/// by `LayerJoin` on the world change, which is what ends a successful transition.
pub(crate) fn end_transition(player: &Player) {
    TRANSITING.with(|set| set.borrow_mut().remove(&key(player)));
}

/// Returns `true` if `player` is in a state where a layer transition must not run.
///
/// Creative and spectator players move freely, so builders can pass through the void without
/// being pulled around. Dead players are skipped because the curse can kill inside the move
/// handler, and the events that follow must not touch a corpse.
#[must_use]
pub(crate) fn ignores_transitions(player: &Player) -> bool {
    exempt_gamemode(player) || is_dead(player)
}
