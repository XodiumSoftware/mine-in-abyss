//! The carried fall distance of a descent that crossed a layer boundary.

use std::cell::RefCell;
use std::collections::HashMap;

use pumpkin_plugin_api::Player;

use super::general::key;
use crate::Layer;

thread_local! {
    /// Fall distances a descent carried into the layer it arrived in, keyed by player id.
    ///
    /// A layer world is only 384 blocks tall, so vanilla resets a player's fall distance
    /// every time a descent swaps worlds. Without this, falling from `First` to `Sixth`
    /// would never reach a damaging distance at all: each segment stays well inside the
    /// height of the world it happens in.
    ///
    /// Written when the swap is issued, and read by
    /// [`FallDamage`](crate::events::fall_damage::FallDamage) on every move in the arrived
    /// layer until the player falls below [`Layer::MIN_Y`](crate::Layer::MIN_Y) — the
    /// point where the landing phase starts and the carry is settled.
    ///
    /// Deliberately **not** persistent: an in-flight fall has no meaning across a restart,
    /// and a stale entry could injure a player with no memory of the drop.
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
/// The carry replaces any previous one: a player can only be in one fall at a time, and
/// the most recent measurement is the one that belongs to the swap about to happen.
pub fn carry_fall(player: &Player, distance: f32, layer: Layer) {
    FALL_CARRY.with(|map| {
        map.borrow_mut()
            .insert(key(player), FallCarry { distance, layer });
    });
}

/// Returns the fall distance `player` should have in `layer` at `y`, given a carried
/// descent.
///
/// `get_fall_distance` is why this is needed at all: the descent teleported the player
/// into a fresh world, where vanilla started counting from zero. The result is the
/// distance so far plus whatever the player has already fallen in this layer, so the
/// landing is the one vanilla eventually sees. Callers apply it themselves.
#[must_use]
pub fn resume_fall(player: &Player, layer: Layer, y: f64) -> Option<f32> {
    let distance = player.as_entity().get_fall_distance();

    if fall_landed(y) {
        take_fall(player);
        return None;
    }

    carried_fall(player, layer, distance).map(|carried| carried + distance)
}

/// Ends `player`'s carried fall, if any.
///
/// Called where a descent cannot continue — a teleport, a respawn, or leaving the Abyss
/// entirely — so a stale distance is never replayed onto a later, unrelated fall.
pub fn clear_fall(player: &Player) {
    take_fall(player);
}

/// Returns `distance` if `player` is carrying a fall that ends in `layer`.
///
/// The carry stays in place: whether the fall is over is decided by [`fall_landed`] on
/// `y`, not by this call, so every move before the landing phase is free to ask again.
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
///
/// Called when the fall stops being a descent — the player reaches the landing phase, is
/// teleported, or the layer below is missing — so a stale distance is never replayed onto
/// a later, unrelated fall.
fn take_fall(player: &Player) -> bool {
    FALL_CARRY.with(|map| map.borrow_mut().remove(&key(player)).is_some())
}

/// Returns `true` once a descent has fallen below [`Layer::MIN_Y`](crate::Layer::MIN_Y),
/// which is where the carry must stop being added.
///
/// Below the floor, a player is on the final drop towards ground, and vanilla's own
/// fall-distance accounting takes over from there. Below the *next* floor they leave the
/// layer instead, and [`LayerLeave`](crate::events::layer_leave::LayerLeave) starts a new
/// segment.
#[must_use]
fn fall_landed(y: f64) -> bool {
    y < Layer::MIN_Y
}
