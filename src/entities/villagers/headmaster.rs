//! Headmaster, The Head Trader, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Headmaster, The Head Trader.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Headmaster;

impl Headmaster {
    /// Spawns Headmaster, The Head Trader in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(78, EntityType::Villager)
            .name(
                TextComponent::text("Headmaster, The Head Trader").color_rgb(rgb(0xC0, 0x39, 0x2B)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cartographer);
                }
            })
            .build(world, position)
    }
}
