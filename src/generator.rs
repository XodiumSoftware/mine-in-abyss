//! The chunk generator used by every layer world.
//!
//! A layer world is empty: the Abyss is a vertical world built from authored maps, so a
//! freshly created world must not come with terrain of its own. Generation therefore
//! writes nothing but air, and does so from the first chunk of the world's life, so no
//! terrain or structure is ever saved to disk to begin with.
//!
//! # Why the air is written explicitly
//!
//! The two surface and feature phases are left at their no-op defaults, which is what
//! keeps grass, ores, trees and structures out of the result. The noise phase is *not*
//! left empty: the buffer a phase receives is reused state, and a buffer that has
//! already been written to would otherwise keep whatever it held. Filling each layer
//! with air makes the empty result independent of whatever the buffer arrived as.

use pumpkin_plugin_api::wit::pumpkin::plugin::biomes::Biome;
use pumpkin_plugin_api::wit::pumpkin::plugin::world::Block;
use pumpkin_plugin_api::worldgen::{ChunkBuffer, ChunkGenerator};
use tracing::warn;

/// Generates empty chunks, one air block at a time.
///
/// Registered once by [`Worlds::setup`](crate::Worlds::setup) and attached to every
/// layer world, so all of them generate the same way.
#[derive(Debug, Default, Clone, Copy)]
pub struct Void;

/// The biome every generated chunk is assigned.
///
/// The biome decides the fog, ambient sound and sky colour a player sees while standing
/// in an otherwise-empty chunk, so the void biome is used: it is the one Minecraft itself
/// pairs with an empty world, and it does not claim the layer is a plains or a forest it
/// has no terrain for. Layers that want their own biome will eventually get it from their
/// authored maps, not from this generator.
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
