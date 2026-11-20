//! A docile wolf of the upper layers.
//!
//! Two parts are not obvious from the code below. The wolf's own goals are cleared first,
//! because a spawned wolf would chase and bite a player and the category says it must not.
//! And every stat but health is authored rather than wiki data: the wiki has no page for this
//! kind, so those are vanilla-wolf figures set explicitly, to be tuned by playtesting.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A docile wolf of the Top of the Abyss and the Forest of Temptation.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Beniguma;

impl Beniguma {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Docile;

    /// Spawns this kind in `world` at `position`.
    ///
    /// Its goals and pathfinding come from the shared `Docile` pass in
    /// [`MobCategory::apply_ai`], so they stay identical to every other docile kind.
    #[must_use]
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(1, EntityType::Wolf)
            .name(TextComponent::text("Beniguma").color_rgb(rgb(0x82, 0x42, 0x43)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 16.0);
                    living.set_health(16.0);
                    living.set_attribute_base(Attribute::AttackDamage, 3.0);
                    living.set_attribute_base(Attribute::MovementSpeed, 0.3);
                    living.set_attribute_base(Attribute::FollowRange, 32.0);
                }

                Beniguma::CATEGORY.apply_ai(entity);
            })
            .build(world, position)
    }
}
