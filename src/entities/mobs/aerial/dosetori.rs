//! The Dosetori, an aerial mob of the Abyss.
//!
//! # Drops
//!
//! The wiki's drop table for this kind, kept as the record to deliver once the
//! plugin API can hook a death. Nothing reads it yet: the host owns the loot of the
//! entity a kind spawns, and no call in the plugin WIT adds or replaces a drop.
//!
//! | Drop | Chance |
//! | ---- | ------ |
//! | Dosetori Meat Raw | 100% |
//! | Dosetori Wing | 70% |
//! | Dosetori Bone | 80% |

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the first layer and the second layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Dosetori;

impl Dosetori {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aerial;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(18, EntityType::Parrot)
            .name(TextComponent::text("Dosetori").color_rgb(rgb(0x59, 0xAC, 0x7A)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 30.0);
                    living.set_health(30.0);
                }
            })
            .build(world, position)
    }
}
