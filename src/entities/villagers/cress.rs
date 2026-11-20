//! Cress, Plant Lover, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Cress, Plant Lover.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cress;

impl Cress {
    /// Spawns Cress, Plant Lover in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(79, EntityType::Villager)
            .name(TextComponent::text("Cress, Plant Lover").color_rgb(rgb(0x4C, 0xAF, 0x50)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Farmer);
                }
            })
            .build(world, position)
    }
}
