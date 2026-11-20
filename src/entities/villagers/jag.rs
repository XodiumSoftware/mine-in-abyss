//! Jag, Protector, a trader of Orth.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Jag, Protector.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Jag;

impl Jag {
    /// Spawns Jag, Protector in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(97, EntityType::Villager)
            .name(TextComponent::text("Jag, Protector").color_rgb(rgb(0xB7, 0x1C, 0x1C)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Armorer);
                }
            })
            .build(world, position)
    }
}
