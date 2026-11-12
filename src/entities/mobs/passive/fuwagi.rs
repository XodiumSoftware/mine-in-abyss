//! The fuwagi, a harmless white rabbit of the first two layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A small white rabbit, built on a rabbit so its skittish wandering comes for free.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Fuwagi;

impl Fuwagi {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Passive;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(13, EntityType::Rabbit)
            .name(TextComponent::text("Fuwagi").color_rgb(rgb(0xE0, 0xE0, 0xE0)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 10.0);
                    living.set_health(10.0);
                }
            })
            .build(world, position)
    }
}
