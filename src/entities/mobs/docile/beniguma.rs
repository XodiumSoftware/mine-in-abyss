//! A docile wolf of the upper layers.
//!
//! Two parts are not obvious from the code below. The wolf's own goals are cleared first,
//! because a spawned wolf would chase and bite a player and the category says it must not.
//! And every stat but health is authored rather than wiki data: the wiki has no page for this
//! kind, so those are vanilla-wolf figures set explicitly, to be tuned by playtesting.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{BuiltinAiGoal, Entity, PathNodeType, World};

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
    /// The wolf's goals are cleared before this kind's are added: the pathfinding maluses
    /// then keep it out of fire, lava and fences.
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

                if let Some(mob) = entity.as_mob() {
                    mob.clear_ai_goals();
                    mob.add_ai_goal(0, BuiltinAiGoal::EscapeDanger(1.2));
                    mob.add_ai_goal(1, BuiltinAiGoal::WanderAround(0.6));
                    mob.add_ai_goal(2, BuiltinAiGoal::LookAtPlayer(8.0));
                    mob.add_ai_goal(3, BuiltinAiGoal::LookAround);

                    mob.set_pathfinding_malus(PathNodeType::DangerFire, 16.0);
                    mob.set_pathfinding_malus(PathNodeType::DamageFire, 16.0);
                    mob.set_pathfinding_malus(PathNodeType::Lava, 32.0);
                    mob.set_pathfinding_malus(PathNodeType::Fence, 8.0);
                }
            })
            .build(world, position)
    }
}
