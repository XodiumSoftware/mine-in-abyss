//! How a mob kind behaves towards players.
//!
//! Kept apart from the traits that answer it because it is the folder layout of
//! `src/entities/mobs`: a kind lives in the folder matching its own `CATEGORY`.

use pumpkin_plugin_api::world::{BuiltinAiGoal, Entity, Mob, PathNodeType};
use strum::{EnumIter, IntoStaticStr};

/// How a mob kind behaves towards players.
///
/// A separate type from [`EntityType`](pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType),
/// which only says *what shape* a mob is: a wolf and a zombie are both spawned as vanilla
/// types, but one is [`MobCategory::Passive`] and the other [`MobCategory::Hostile`]. The
/// category is what a spawn rule, a difficulty tweak or a bestiary entry reads when the vanilla
/// type is not the question.
///
/// Its `id()` is a fixed `snake_case` spelling rather than the variant name, so `"passive"`
/// stays readable in a command argument, a data file or a log line, the same way
/// [`Layer::id`](crate::layer::Layer) does it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MobCategory {
    /// Never attacks a player unprovoked; will fight back or not, per kind.
    #[default]
    Passive,
    /// May be bred or tamed, and only harms players when provoked.
    Docile,
    /// Moves through the air, so its pathing ignores the ground.
    Aerial,
    /// Swims, so it belongs in water rather than on the layer's map.
    Aquatic,
    /// Attacks players on sight.
    Hostile,
}

impl MobCategory {
    /// Every category, in the order used for listings.
    pub const ALL: [Self; 5] = [
        Self::Passive,
        Self::Docile,
        Self::Aerial,
        Self::Aquatic,
        Self::Hostile,
    ];

    /// This category's stable lowercase identifier, e.g. `"hostile"`.
    ///
    /// Read off the variant by `strum`, never hand-written, so renaming a variant cannot
    /// leave a stale spelling behind.
    #[must_use]
    pub fn id(self) -> &'static str {
        self.into()
    }

    /// Returns whether a mob of this category hunts players.
    #[must_use]
    pub fn is_hostile(self) -> bool {
        matches!(self, Self::Hostile)
    }

    /// Applies the goals and pathfinding every kind of this category shares.
    ///
    /// A spawned vanilla type arrives with its own goals, so they are cleared first: a wolf
    /// would hunt and a phantom would swoop whatever the category promises. What the
    /// category then adds is the small set of [`BuiltinAiGoal`]s that express its disposition.
    /// **Custom goals are unreachable** — the registry behind `add_custom_ai_goal` is
    /// `pub(crate)` — so this is the whole vocabulary available.
    ///
    /// A hostile kind gets no melee goal here: `zombie_attack`, `blaze_attack` and
    /// `creeper_ignite` are specific to the vanilla types that read them, and the base type
    /// already brings its own attack. What is added is the awareness the cleared set took
    /// away, so it still notices and closes on a player.
    pub(crate) fn apply_ai(self, entity: &Entity) {
        let Some(mob) = entity.as_mob() else { return };

        mob.clear_ai_goals();
        mob.add_ai_goal(1, BuiltinAiGoal::WanderAround(0.6));
        mob.add_ai_goal(3, BuiltinAiGoal::LookAtPlayer(8.0));
        mob.add_ai_goal(4, BuiltinAiGoal::LookAround);

        match self {
            Self::Hostile => {
                mob.add_ai_goal(0, BuiltinAiGoal::MeleeAttack(1.0));
                mob.set_pathfinding_malus(PathNodeType::DangerFire, 4.0);
            }
            Self::Aerial => {
                mob.add_ai_goal(0, BuiltinAiGoal::MeleeAttack(1.0));
            }
            Self::Aquatic => mob.add_ai_goal(0, BuiltinAiGoal::Swim),
            Self::Docile => {
                mob.add_ai_goal(0, BuiltinAiGoal::EscapeDanger(1.2));
                mob.add_ai_goal(2, BuiltinAiGoal::MeleeAttack(1.0));
            }
            Self::Passive => {
                mob.add_ai_goal(0, BuiltinAiGoal::EscapeDanger(1.2));
            }
        }

        self.apply_pathfinding(&mob);
    }

    /// Keeps a land kind out of terrain it should not walk into.
    ///
    /// Only `Docile` and `Passive` are tuned, since those are the kinds that flee and wander
    /// across open ground; a hostile kind is chasing, and an aquatic or aerial one does not
    /// path over fire at all. The values are relative costs, not distances, so they say
    /// "avoid" rather than "forbidden" and are a starting guess to tune in play.
    fn apply_pathfinding(self, mob: &Mob) {
        if !matches!(self, Self::Passive | Self::Docile) {
            return;
        }

        mob.set_pathfinding_malus(PathNodeType::DangerFire, 16.0);
        mob.set_pathfinding_malus(PathNodeType::DamageFire, 16.0);
        mob.set_pathfinding_malus(PathNodeType::Lava, 32.0);
        mob.set_pathfinding_malus(PathNodeType::Fence, 8.0);
    }
}
