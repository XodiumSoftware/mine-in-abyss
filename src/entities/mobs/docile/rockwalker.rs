//! The Rockwalker, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the second layer and the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Rockwalker;

impl Rockwalker {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::Second, Layer::Third];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Docile;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(22, EntityType::Silverfish)
            .name(TextComponent::text("Rockwalker").color_rgb(rgb(0xBE, 0x64, 0x00)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 25.0);
                    living.set_health(25.0);
                }
            })
            .build(world, position)
    }
}
