//! Makes monsters exponentially harder the deeper the layer they are in.
//!
//! A layer's danger is its [`Layer::monster_scale`]: `1x` at the surface, doubling with
//! every layer down, so `Seventh` monsters are `64x` their surface stats. The curve is
//! derived from depth exactly like the curse, so a layer cannot be forgotten and the
//! difficulty can never drift from the enum order.
//!
//! # What is scaled
//!
//! Every monster is rescaled on `EntitySpawnEvent`, which the server fires once an entity
//! exists and before it can be fought:
//!
//! - **Health** — the `MaxHealth` base and the current health are both multiplied, so a
//!   monster does not arrive still holding a surface-sized health pool.
//! - **Attack damage** — the stat that decides whether a layer is survivable; without it a
//!   `64x`-health zombie would only take longer to kill.
//! - **Follow range** — keeps the harder monsters from being outrun. A monster that can
//!   never reach the player is not harder, only slower.
//!
//! # Which spawns count
//!
//! `EntitySpawnEvent` is the only spawn event every mob passes through: natural spawns,
//! spawners and `/summon` all reach it, so all three are scaled. The surface is skipped
//! (`1x`), as is anything that is not a monster, so players, animals and dropped items are
//! untouched.
//!
//! A monster that follows a descending player never spawns at all, so it is rescaled by
//! [`Worlds::descend_monsters`](crate::Worlds::descend_monsters) on arrival instead — the
//! same [`Monsters::apply`], so what follows a player is exactly as dangerous as the layer
//! it followed them into.

use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::entity::entity_spawn::EntitySpawnEvent;
use pumpkin_plugin_api::events_wit::EntitySpawnEventData;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;
use pumpkin_plugin_api::{Context, Server, World};

use crate::events::event::Event;
use crate::layer::Layer;

/// Multiplies the health, attack damage and follow range of every monster entering a layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Monsters;

impl Event for Monsters {
    fn register(context: &Context) {
        context
            .register_event_handler::<EntitySpawnEvent, _>(Monsters, Self::PRIORITY, Self::BLOCKING)
            .unwrap_or_else(|_| Self::fail("Monsters"));
    }
}

impl Monsters {
    /// Applies `layer`'s monster scale to a living entity.
    ///
    /// Public because a monster that follows a descent arrives through
    /// [`Worlds::descend_monsters`](crate::Worlds::descend_monsters) rather than a spawn
    /// event, and must be rescaled for the layer it lands in.
    ///
    /// The multiplier is applied to the **base** value, so rescaling an entity that is
    /// already scaled is safe: the base is the template and the visible value stays
    /// `base * scale`, rather than compounding on every call.
    pub fn apply(living: &LivingEntity, layer: Layer) {
        let scale = layer.monster_scale();
        if scale <= 1.0 {
            return;
        }

        let max_health = f64::from(living.get_max_health()) * scale;
        let health = f64::from(living.get_health()) * scale;

        living.set_attribute_base(Attribute::MaxHealth, max_health);
        living.set_max_health(as_health(max_health));
        living.set_health(as_health(health));

        scale_attribute(living, Attribute::AttackDamage, scale);
        scale_attribute(living, Attribute::FollowRange, scale);
    }
}

impl EventHandler<EntitySpawnEvent> for Monsters {
    fn handle(&self, _server: Server, event: EntitySpawnEventData) -> EntitySpawnEventData {
        let Some(layer) = crate::layer_for_world(&event.target_world.get_name()) else {
            return event;
        };

        if layer.monster_scale() <= 1.0 {
            return event;
        }

        let Some(entity_type) = monster_type(&event.entity_type) else {
            return event;
        };

        if let Some(entity) = spawned_entity(&event.target_world, entity_type, &event)
            && let Some(living) = entity.as_living()
        {
            Self::apply(&living, layer);
        }

        event
    }
}

/// Returns the entity the spawn event just created.
///
/// The event identifies it only by a numeric id, and the plugin API offers no lookup by
/// id or UUID, so the entity is found through its world: it has just spawned, so it is the
/// one of its type standing at that exact position.
fn spawned_entity(
    world: &World,
    entity_type: EntityType,
    event: &EntitySpawnEventData,
) -> Option<pumpkin_plugin_api::Entity> {
    world.get_entities().into_iter().find(|candidate| {
        candidate.get_type() == entity_type && candidate.get_position() == event.position
    })
}

/// Multiplies `attribute`'s base value by `scale`, leaving it alone if the entity has none.
///
/// Vanilla reports `0` for an attribute an entity does not have — a passive animal has no
/// attack damage — and scaling zero would keep it at zero anyway, so the guard avoids
/// writing a meaningless value rather than guarding against an error.
fn scale_attribute(living: &LivingEntity, attribute: Attribute, scale: f64) {
    let base = living.get_attribute_base(attribute);
    if base > 0.0 {
        living.set_attribute_base(attribute, base * scale);
    }
}

/// Clamps a scaled health value into the range a health pool accepts.
///
/// A `64x` multiplier can push a value outside what an `f32` health pool holds, and a
/// value that does not fit would be worse than a clamped one: the monster would be
/// unkillable or already dead. The cast is unchecked because the standard library has no
/// checked `f64 -> f32` conversion.
fn as_health(value: f64) -> f32 {
    #[allow(clippy::cast_possible_truncation)]
    let health = value as f32;
    health.max(1.0)
}

/// Returns the monster the spawn event's type string refers to, or `None` for anything
/// that is not a monster.
///
/// **The host sends a numeric id here, not a name.** Pumpkin builds the field as
/// `entity_type.id.to_string()`, where `id` is the registry index in its
/// `pumpkin-data` entity type table — `zombie` is `"154"`, `cave_spider` is `"22"`. The
/// `minecraft:` prefix and the `resource_name` form are both accepted as well, because
/// they cost nothing and the alternative is silently matching nothing.
///
/// The ids are therefore tied to Pumpkin's registry order and must be updated if it is
/// renumbered. [`MONSTERS`] is the single place they live, so a renumbering is a one-line
/// change per entry; there is no name accessor on [`EntityType`] to derive them from.
fn monster_type(name: &str) -> Option<EntityType> {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    MONSTERS.iter().copied().find(|monster| {
        let id = monster_id(*monster);
        id.pumpkin_id.to_string() == name || id.resource_name == name
    })
}

/// Returns `true` if `entity_type` is a hostile monster whose stats are scaled by depth.
///
/// An allow-list rather than "everything living": the Abyss should get harder, not weird,
/// and a player's horse or a village's villagers must keep their stats. A mob that should
/// become part of the Abyss belongs in [`MONSTERS`].
#[must_use]
pub fn is_monster(entity_type: EntityType) -> bool {
    MONSTERS.contains(&entity_type)
}

/// The hostile mobs whose stats are scaled by depth.
const MONSTERS: [EntityType; 13] = [
    EntityType::Blaze,
    EntityType::CaveSpider,
    EntityType::Creeper,
    EntityType::Drowned,
    EntityType::Enderman,
    EntityType::Ghast,
    EntityType::Husk,
    EntityType::Phantom,
    EntityType::Skeleton,
    EntityType::Slime,
    EntityType::Spider,
    EntityType::WitherSkeleton,
    EntityType::Zombie,
];

/// How Pumpkin identifies a monster: its registry index and its resource name.
///
/// Kept local because the enum exposes no accessor for either. Listing both keeps the
/// match honest whichever form a host sends, and `MONSTERS` keeps this in step by
/// construction — adding a monster needs an entry here.
struct MonsterId {
    /// The value of `pumpkin_data`'s `EntityType::id`, sent by the host as the type string.
    pumpkin_id: u16,
    /// The `resource_name`, which a host that names the type would send instead.
    resource_name: &'static str,
}

/// The identifiers a spawn event uses for `entity_type`.
///
/// The match is exhaustive over [`MONSTERS`]: there is deliberately no catch-all, so a
/// monster added to the list without an id here fails to compile rather than silently
/// never matching a spawn.
fn monster_id(entity_type: EntityType) -> MonsterId {
    match entity_type {
        EntityType::Blaze => MonsterId {
            pumpkin_id: 14,
            resource_name: "blaze",
        },
        EntityType::CaveSpider => MonsterId {
            pumpkin_id: 22,
            resource_name: "cave_spider",
        },
        EntityType::Creeper => MonsterId {
            pumpkin_id: 32,
            resource_name: "creeper",
        },
        EntityType::Drowned => MonsterId {
            pumpkin_id: 39,
            resource_name: "drowned",
        },
        EntityType::Enderman => MonsterId {
            pumpkin_id: 42,
            resource_name: "enderman",
        },
        EntityType::Ghast => MonsterId {
            pumpkin_id: 58,
            resource_name: "ghast",
        },
        EntityType::Husk => MonsterId {
            pumpkin_id: 68,
            resource_name: "husk",
        },
        EntityType::Phantom => MonsterId {
            pumpkin_id: 100,
            resource_name: "phantom",
        },
        EntityType::Skeleton => MonsterId {
            pumpkin_id: 118,
            resource_name: "skeleton",
        },
        EntityType::Slime => MonsterId {
            pumpkin_id: 120,
            resource_name: "slime",
        },
        EntityType::Spider => MonsterId {
            pumpkin_id: 127,
            resource_name: "spider",
        },
        EntityType::WitherSkeleton => MonsterId {
            pumpkin_id: 150,
            resource_name: "wither_skeleton",
        },
        EntityType::Zombie => MonsterId {
            pumpkin_id: 154,
            resource_name: "zombie",
        },
        _ => {
            // Unreachable: `monster_id` is only called on members of `MONSTERS`. The arm
            // exists because `EntityType` has 161 variants and the enum cannot be
            // narrowed, so an impossible id is returned instead of panicking.
            MonsterId {
                pumpkin_id: u16::MAX,
                resource_name: "",
            }
        }
    }
}
