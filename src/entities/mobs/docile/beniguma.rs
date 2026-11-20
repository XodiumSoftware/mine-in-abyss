//! A docile wolf of the upper layers, showing what a custom mob can express.
//!
//! This is the worked example for a mob kind: it states its [`LAYERS`](Beniguma::LAYERS),
//! [`CATEGORY`](Beniguma::CATEGORY) and `spawn`, and sets its health and its goals in one
//! `apply` — which is how every kind defines itself, with no fixed table of multipliers or
//! slots to fill in.
//!
//! In the Abyss this is a docile wolf of the first and second layers: tougher than a plain
//! wolf, so a player can tell it apart from the vanilla mob it is built on and thinks twice
//! before provoking one.
//!
//! # Goals
//!
//! Its behaviour is authored from [`BuiltinAiGoal`]s, and it is the worked example for that
//! too. A wolf spawns with its own melee, target and breeding goals, which would make a
//! "docile" creature chase a player down — so this kind clears them first and then adds only
//! the goals a docile creature should have: wander, look around, and flee. No melee goal and
//! no targeting means it cannot fight back, which is the behaviour the category promises.
//!
//! `Entity::as_mob` is `Some` for a spawned vanilla mob, which a wolf is; a base type without
//! AI would return `None` and the kind would keep whatever the host gave it.
//!
//! **Custom goals are not reachable yet.** The API declares the `AiGoal` trait and the host
//! exports `handle-ai-goal-*`, but the registry that maps a goal id to a goal is `pub(crate)`,
//! so nothing outside the crate can put one behind `add_custom_ai_goal`. A kind therefore uses
//! [`BuiltinAiGoal`]s only, and gains a custom goal when upstream exposes a register call.
//!
//! # Drops
//!
//! The wiki's drop table for this kind, kept as the record to deliver once the
//! plugin API can hook a death. Nothing reads it yet: the host owns the loot of the
//! entity a kind spawns, and no call in the plugin WIT adds or replaces a drop.
//!
//! | Drop | Chance |
//! | ---- | ------ |
//! | Beniguma Fur | 60% |
//! | Sweet Berries | 60% |

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
    /// The wolf's own goals are cleared before this kind's are added: a vanilla wolf would
    /// hunt skeletons and sheep and turn on a player who hits it, which is not what a docile
    /// creature of the Abyss does. What is left is flight, wandering and looking about.
    ///
    /// # The numbers
    ///
    /// Health is the wiki's 16. **The other attributes are authored, not wiki data**: the
    /// wiki has no page for this kind and no stat block for it, so `AttackDamage`,
    /// `MovementSpeed` and `FollowRange` are set to vanilla-wolf figures rather than invented.
    /// Each is written explicitly even where it matches the base, because the base is a wolf's
    /// and a later change to the creature should not be silently inherited from it.
    /// `FollowRange` is the widest departure: it is raised so the creature notices a player
    /// across a clearing, while the short `AttackDamage` says it is not a threat — the two
    /// together are what makes it read as docile rather than broken.
    ///
    /// The pathfinding maluses are the other half of that: a docile creature of the upper
    /// layers should not walk into fire, lava or a drop, and it should not pick its way
    /// through a fence. Raising a malus makes a route through that terrain costlier, so the
    /// host's pathfinder avoids it without any custom AI.
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
