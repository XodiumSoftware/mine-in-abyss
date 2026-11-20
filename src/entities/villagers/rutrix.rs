//! Rutrix, DJ, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Rutrix, DJ.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Rutrix;

impl Rutrix {
    /// Spawns Rutrix, DJ in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(77, EntityType::Villager)
            .name(TextComponent::text("Rutrix, DJ").color_rgb(rgb(0x9B, 0x30, 0xD8)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cartographer);
                }
            })
            .build(world, position)
    }
}
