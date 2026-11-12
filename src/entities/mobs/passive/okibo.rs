//! The okibo, a large passive grazer of Orth and the first layer.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A big green sniffer-like creature, found in the city and just below it.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Okibo;

impl Okibo {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::Orth, Layer::First];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Passive;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(14, EntityType::Sniffer)
            .name(TextComponent::text("Okibo").color_rgb(rgb(0x10, 0xDE, 0xA7)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 40.0);
                    living.set_health(40.0);
                }
            })
            .build(world, position)
    }
}
