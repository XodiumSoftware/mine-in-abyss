//! Jeff, Enchanter, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Jeff, Enchanter.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Jeff;

impl Jeff {
    /// Spawns Jeff, Enchanter in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(96, EntityType::Villager)
            .name(TextComponent::text("Jeff, Enchanter").color_rgb(rgb(0x8E, 0x44, 0xAD)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Weaponsmith);
                }
            })
            .build(world, position)
    }
}
