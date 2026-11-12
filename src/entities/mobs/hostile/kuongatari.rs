//! The Kuongatari, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Kuongatari;

impl Kuongatari {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::Fourth, Layer::Fifth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Hostile;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(41, EntityType::Spider)
            .name(TextComponent::text("Kuongatari").color_rgb(rgb(0x1F, 0xB5, 0x3D)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 30.0);
                    living.set_health(30.0);
                }
            })
            .build(world, position)
    }
}
