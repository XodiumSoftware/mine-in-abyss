//! Benner, Also Lord of Currency, a trader of Orth.
//!
//! Boy's companion at the **Golden Bridge**, and the other Lord of Currency.
//!
//! The wiki's trade table for this counter is deliberately **not** written here: the plugin API
//! has no call that sets a villager's offers, so a table would be data nothing could deliver.
//! What is stated is the shape of the trader - its profession, so the work sound and badge are
//! right, and its name, so a player can tell it from a village villager.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Benner, Also Lord of Currency.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Benner;

impl Benner {
    /// Spawns Benner, Also Lord of Currency in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(67, EntityType::Villager)
            .name(
                TextComponent::text("Benner, Also Lord of Currency")
                    .color_rgb(rgb(0xF0, 0xC4, 0x38)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Librarian);
                }
            })
            .build(world, position)
    }
}
