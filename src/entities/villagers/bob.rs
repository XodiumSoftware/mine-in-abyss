//! Bob, Handyman, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Bob, Handyman.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Bob;

impl Bob {
    /// Spawns Bob, Handyman in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(82, EntityType::Villager)
            .name(TextComponent::text("Bob, Handyman").color_rgb(rgb(0xFF, 0x8C, 0x00)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Toolsmith);
                }
            })
            .build(world, position)
    }
}
