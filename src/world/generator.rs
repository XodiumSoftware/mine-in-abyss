//! The chunk generator used by every layer world.
//!
//! A layer world is empty: the Abyss is built from authored maps, so a fresh world must not
//! arrive with terrain of its own. Leaving the surface and feature phases at their no-op
//! defaults keeps grass, ores, trees and structures out; the noise phase fills each chunk with
//! air explicitly, because the buffer a phase receives is reused state and would otherwise
//! keep whatever it held.

use pumpkin_plugin_api::wit::pumpkin::plugin::biomes::Biome;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::Block;
use pumpkin_plugin_api::worldgen::{ChunkBuffer, ChunkGenerator};
use tracing::warn;

/// Generates empty chunks, one air block at a time.
///
/// Registered once by [`Worlds::setup`](super::setup::Worlds::setup) and attached to every
/// layer world.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Void;

/// The biome every generated chunk is assigned: the one Minecraft pairs with an empty world,
/// so the fog and sky do not claim terrain the layer has none of.
const EMPTY_BIOME: Biome = Biome::TheVoid;

impl ChunkGenerator for Void {
    fn generate_biomes(&self, chunk: &mut ChunkBuffer) {
        chunk.fill_biome(EMPTY_BIOME);
    }

    fn generate_noise(&self, chunk: &mut ChunkBuffer) {
        let Some(air) = empty_state_id() else {
            return;
        };

        let min_y = chunk.min_y();
        let max_y = min_y.saturating_add(chunk.height().cast_signed());

        for y in min_y..max_y {
            chunk.fill_layer(y, air);
        }
    }
}

/// Returns the block state id of air, or logs its absence.
///
/// [`ChunkBuffer`] takes raw numeric state ids, so the id has to be resolved from the
/// registry rather than assumed: a hardcoded id would silently fill every chunk with
/// whichever block happened to own that number. Resolution is a server call, so it is
/// done once per chunk instead of once per block.
fn empty_state_id() -> Option<u16> {
    const EMPTY_BLOCK: &str = "minecraft:air";

    let block = Block::from_name(EMPTY_BLOCK);
    if block.is_none() {
        warn!("Block '{EMPTY_BLOCK}' is unknown; generated chunks will be left unset");
    }
    block.map(|block| block.default_state_id)
}
