//! The splitjaw, the heaviest flyer in the upper half of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A large purple flyer, met from the Top of the Abyss down to the Great Fault.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Splitjaw;

impl Splitjaw {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second, Layer::Third];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aerial;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(5, EntityType::Parrot)
            .name(TextComponent::text("Splitjaw").color_rgb(rgb(0x9A, 0x1C, 0xAD)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 90.0);
                    living.set_health(90.0);
                }
            })
            .build(world, position)
    }
}
