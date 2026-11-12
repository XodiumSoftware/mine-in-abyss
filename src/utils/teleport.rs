//! Recognising a move that merely reports where a teleport put the player.

use std::cell::RefCell;
use std::collections::HashMap;

use pumpkin_plugin_api::Player;

use super::general::key;

thread_local! {
    /// Players who arrived at their current position by teleport, keyed by player id.
    ///
    /// A teleport is not a climb, so the first move event after one must not be read as
    /// an ascent out of the Abyss. `LayerTeleport` records the destination, and
    /// [`consume_teleport`] hands that Y back once so `LayerLeave` can ignore it.
    ///
    /// Consumed on first read, so it cannot suppress a real ascent later.
    static TELEPORTED: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
}

/// How far a move may differ from a teleport destination and still count as the same
/// arrival, in blocks. Covers the drift between the requested and applied position.
const TELEPORT_TOLERANCE: f64 = 2.0;

/// Records that `player` was teleported to `y`, so the next move is not read as a climb.
pub(crate) fn note_teleport(player: &Player, y: f64) {
    TELEPORTED.with(|map| map.borrow_mut().insert(key(player), y));
}

/// Returns the Y a teleport put `player` at, clearing the record.
///
/// Returns `None` when the player reached their position by moving normally.
#[must_use]
pub(crate) fn consume_teleport(player: &Player) -> Option<f64> {
    TELEPORTED.with(|map| map.borrow_mut().remove(&key(player)))
}

/// Returns `true` if this move merely reports a position a teleport already placed the
/// player at.
///
/// Being teleported above the ceiling is not an ascent, and must not be cursed. The
/// record is consumed either way, so a genuine climb afterwards still counts; the Y
/// comparison stops a stale record swallowing an unrelated move.
#[must_use]
pub(crate) fn arrived_by_teleport(player: &Player, y: f64) -> bool {
    consume_teleport(player)
        .is_some_and(|teleported_y| (teleported_y - y).abs() < TELEPORT_TOLERANCE)
}
