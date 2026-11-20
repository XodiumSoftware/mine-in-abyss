//! Boy, Lord of Currency, and Orth's relic buyer.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Boy, Lord of Currency.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Boy;

impl Boy {
    /// Spawns Boy in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(66, EntityType::Villager)
            .name(TextComponent::text("Boy, Lord of Currency").color_rgb(rgb(0xF0, 0xC4, 0x38)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Librarian);
                }
            })
            .build(world, position)
    }
}
