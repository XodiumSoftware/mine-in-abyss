//! Resolving a layer from a world, and a world from a layer.

use pumpkin_plugin_api::{Player, Server, World};
use tracing::warn;

use crate::Layer;

/// Resolves the world for `layer`, logging `purpose` when it cannot be found.
///
/// A missing world means [`Worlds::setup`](crate::Worlds::setup) failed for that layer. The
/// caller says what it was trying to do, so the log explains why it mattered — a player
/// stuck in the void, a clock that cannot be synced and a respawn that went elsewhere are
/// different failures.
///
/// Every caller returns quietly on `None`: one unavailable world must not take down an
/// event handler or a repeating task that still has other layers to serve.
pub(crate) fn world_for(server: &Server, layer: Layer, purpose: &str) -> Option<World> {
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
pub(crate) fn layer_of(player: &Player) -> Option<Layer> {
    crate::layer_for_world(&player.get_world().get_name())
}
