//! The carried fall distance of a descent that crossed a layer boundary.

use std::cell::RefCell;
use std::collections::HashMap;

use pumpkin_plugin_api::Player;

use super::general::key;
use crate::Layer;

thread_local! {
    /// Fall distances a descent carried into the layer it arrived in, keyed by player id.
    ///
    /// Vanilla resets a player's fall distance when a descent swaps worlds, and a layer is
    /// only 384 blocks tall, so per-world falls would never reach a damaging distance. The
    /// carry is settled once the player falls below [`Layer::MIN_Y`], where vanilla's own
    /// accounting takes over.
    ///
    /// Deliberately not persistent: an in-flight fall has no meaning across a restart, and a
    /// stale entry could injure a player with no memory of the drop.
    static FALL_CARRY: RefCell<HashMap<String, FallCarry>> = RefCell::new(HashMap::new());
}

/// How far a player had fallen when a descent swapped them into the next layer.
#[derive(Debug, Clone, Copy)]
struct FallCarry {
    /// The fall distance measured before the world change, in blocks.
    distance: f32,
    /// The layer the descent arrived in, which owns the landing.
    layer: Layer,
}

/// Records that `player` fell `distance` blocks before a descent into `layer`.
///
/// Replaces any previous carry: a player can only be in one fall at a time.
pub(crate) fn carry_fall(player: &Player, distance: f32, layer: Layer) {
    FALL_CARRY.with(|map| {
        map.borrow_mut()
            .insert(key(player), FallCarry { distance, layer });
    });
}

/// Returns the fall distance `player` should have in `layer` at `y`: the part carried plus
/// whatever they have already fallen here, so the landing is the one vanilla eventually sees.
#[must_use]
pub(crate) fn resume_fall(player: &Player, layer: Layer, y: f64) -> Option<f32> {
    let distance = player.as_entity().get_fall_distance();

    if fall_landed(y) {
        take_fall(player);
        return None;
    }

    carried_fall(player, layer, distance).map(|carried| carried + distance)
}

/// Ends `player`'s carried fall, if any. Called where a descent cannot continue, so a stale
/// distance is never replayed onto a later, unrelated fall.
pub(crate) fn clear_fall(player: &Player) {
    take_fall(player);
}

/// Returns the carry if it ends in `layer` and is larger than what the player has fallen
/// here. The carry stays in place: whether the fall is over is decided by [`fall_landed`].
#[must_use]
fn carried_fall(player: &Player, layer: Layer, distance: f32) -> Option<f32> {
    FALL_CARRY.with(|map| {
        map.borrow()
            .get(&key(player))
            .filter(|carry| carry.layer == layer && carry.distance > distance)
            .map(|carry| carry.distance)
    })
}

/// Clears `player`'s carried fall, returning `true` if there was one.
fn take_fall(player: &Player) -> bool {
    FALL_CARRY.with(|map| map.borrow_mut().remove(&key(player)).is_some())
}

/// Returns `true` once a descent has fallen below [`Layer::MIN_Y`], where the carry must stop
/// being added: vanilla's own fall accounting takes over from there.
#[must_use]
fn fall_landed(y: f64) -> bool {
    y < Layer::MIN_Y
}
