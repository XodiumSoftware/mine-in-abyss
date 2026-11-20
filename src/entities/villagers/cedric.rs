//! Cedric, Cementer, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Cedric, Cementer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cedric;

impl Cedric {
    /// Spawns Cedric, Cementer in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(74, EntityType::Villager)
            .name(TextComponent::text("Cedric, Cementer").color_rgb(rgb(0xB0, 0xB0, 0xB0)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
