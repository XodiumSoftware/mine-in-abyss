//! The Stinger, an aerial mob of the Abyss.
//!
//! # Drops
//!
//! The wiki's drop table for this kind, kept as the record to deliver once the
//! plugin API can hook a death. Nothing reads it yet: the host owns the loot of the
//! entity a kind spawns, and no call in the plugin WIT adds or replaces a drop.
//!
//! | Drop | Chance |
//! | ---- | ------ |
//! | Stinger Wing | 50% |
//! | Stinger Horn | 20% |

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the second layer and the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Stinger;

impl Stinger {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::Second, Layer::Fourth, Layer::Fifth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aerial;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(24, EntityType::Bee)
            .name(TextComponent::text("Stinger").color_rgb(rgb(0x03, 0xAC, 0x13)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 16.0);
                    living.set_health(16.0);
                }
            })
            .build(world, position)
    }
}
