//! Small helpers shared across the plugin's events and layers.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::{GameMode, NamedColor, RgbColor};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::uuid;
use pumpkin_plugin_api::{Server, World};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use tracing::warn;

use crate::Layer;

thread_local! {
    /// Players currently being moved between layers, keyed by player id.
    ///
    /// `PlayerMoveEvent` fires on every movement, so a single transition would
    /// otherwise re-trigger on each tick the player spends inside the void or ceiling
    /// band — teleporting them repeatedly and adding curse strain each time. This
    /// latch makes a transition one-shot.
    ///
    /// It is held from the moment the teleport is issued until the player arrives in the
    /// destination world (see [`arrive_transition`]), not just for the remainder of the
    /// handler: the world change and the moves around it are not guaranteed to finish
    /// inside one event, so releasing immediately would let the tail of the transition
    /// re-trigger in the new world.
    ///
    /// Deliberately **not** persistent: it only describes an in-flight transition, and
    /// a stale entry after a restart would lock a player out of transitions forever.
    static TRANSITING: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

    /// Players who arrived at their current position by teleport, keyed by player id.
    ///
    /// A teleport is not a climb, so the first move event after one must not be read as
    /// an ascent out of the Abyss. `LayerTeleport` records the destination, and
    /// [`consume_teleport`] hands that Y back once so `LayerLeave` can ignore it.
    ///
    /// Consumed on first read, so it cannot suppress a real ascent later.
    static TELEPORTED: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());

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
    layer: crate::Layer,
}

/// Returns a stable key for `player`, usable across separate WIT handles.
fn key(player: &Player) -> String {
    uuid::to_string(player.get_id())
}

/// Resolves the world for `layer`, logging `purpose` when it cannot be found.
///
/// A missing world means [`Worlds::setup`](crate::Worlds::setup) failed for that layer. The
/// caller says what it was trying to do, so the log explains why it mattered — a player
/// stuck in the void, a clock that cannot be synced and a respawn that went elsewhere are
/// different failures.
///
/// Every caller returns quietly on `None`: one unavailable world must not take down an
/// event handler or a repeating task that still has other layers to serve.
pub fn world_for(server: &Server, layer: Layer, purpose: &str) -> Option<World> {
    let name = layer.world();
    let world = server.get_world_by_name(&name);
    if world.is_none() {
        warn!("World '{name}' is unavailable; {purpose}");
    }
    world
}

/// Returns the layer `player` is standing in, if any.
///
/// Every handler that reacts to a player has to answer this first, and each one would
/// otherwise repeat the same two steps: read the current world's name, then match it
/// against [`Layer::world`](crate::Layer::world). Reading the world here rather than taking
/// a name keeps callers from having to re-resolve it for a field they do not use.
#[must_use]
pub fn layer_of(player: &Player) -> Option<Layer> {
    crate::layer_for_world(&player.get_world().get_name())
}

/// Builds an [`RgbColor`] from its channels, so palettes can be written inline.
#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
    RgbColor { r, g, b }
}

/// Renders `number` with its English ordinal suffix (`1st`, `2nd`, `3rd`, `4th`, …).
///
/// Callers pass what they already have — a layer's [`number`](crate::Layer::number), for
/// instance — rather than the plugin storing a formatted ordinal anywhere, so a renamed
/// or reordered list cannot leave a stale suffix behind.
#[must_use]
pub fn ordinal(number: usize) -> String {
    let suffix = match number {
        11..=13 => "th",
        _ => match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{number}{suffix}")
}

/// Records that `player` was teleported to `y`, so the next move is not read as a climb.
pub fn note_teleport(player: &Player, y: f64) {
    TELEPORTED.with(|map| map.borrow_mut().insert(key(player), y));
}

/// Returns the Y a teleport put `player` at, clearing the record.
///
/// Returns `None` when the player reached their position by moving normally.
#[must_use]
pub fn consume_teleport(player: &Player) -> Option<f64> {
    TELEPORTED.with(|map| map.borrow_mut().remove(&key(player)))
}

/// Records that `player` fell `distance` blocks before a descent into `layer`.
///
/// The carry replaces any previous one: a player can only be in one fall at a time, and
/// the most recent measurement is the one that belongs to the swap about to happen.
pub fn carry_fall(player: &Player, distance: f32, layer: crate::Layer) {
    FALL_CARRY.with(|map| {
        map.borrow_mut()
            .insert(key(player), FallCarry { distance, layer });
    });
}

/// Returns `distance` if `player` is carrying a fall that ends in `layer`.
///
/// The carry stays in place: whether the fall is over is decided by [`fall_landed`] on
/// `y`, not by this call, so every move before the landing phase is free to ask again.
#[must_use]
pub fn carried_fall(player: &Player, layer: crate::Layer, distance: f32) -> Option<f32> {
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
pub fn take_fall(player: &Player) -> bool {
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
    y < crate::Layer::MIN_Y
}

/// Returns the fall distance `player` should have in `layer` at `y`, given a carried
/// descent.
///
/// `get_fall_distance` is why this is needed at all: the descent teleported the player
/// into a fresh world, where vanilla started counting from zero. The result is the
/// distance so far plus whatever the player has already fallen in this layer, so the
/// landing is the one vanilla eventually sees. Callers apply it themselves.
#[must_use]
pub fn resume_fall(player: &Player, layer: crate::Layer, y: f64) -> Option<f32> {
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
    let exempt_gamemode = matches!(
        player.get_gamemode(),
        GameMode::Creative | GameMode::Spectator
    );

    exempt_gamemode || is_dead(player)
}

/// Tells `player` something with a short message in the action bar.
///
/// The action bar rather than chat: these are answers to one interaction — a refused
/// spawn point, a layer with nothing beside it — and should not clutter the log. `color`
/// carries the meaning, red for a refusal and grey for a neutral explanation.
pub fn tell(player: &Player, message: &str, color: NamedColor) {
    player.show_actionbar(TextComponent::text(message).color_named(color));
}

/// Returns `true` if `player` has died and has not respawned yet.
#[must_use]
fn is_dead(player: &Player) -> bool {
    player
        .as_entity()
        .as_living()
        .is_some_and(|living| living.is_dead())
}
