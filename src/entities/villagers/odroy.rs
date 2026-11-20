//! Odroy, The Apothecary, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Odroy, The Apothecary.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Odroy;

impl Odroy {
    /// Spawns Odroy, The Apothecary in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(68, EntityType::Villager)
            .name(TextComponent::text("Odroy, The Apothecary").color_rgb(rgb(0x66, 0xCC, 0x66)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cleric);
                }
            })
            .build(world, position)
    }
}
