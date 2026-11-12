//! The trait every custom entity implements, whatever kind of entity it is.
//!
//! Minecraft's entity types cannot be extended: an entity spawned through this plugin is a
//! vanilla type to the server. The only thing that distinguishes it from a naturally
//! spawned one is a marker in its own persistent data, written once at spawn and read by
//! every handler that needs to know what it is looking at.
//!
//! The marker is [`EntityBuilder::id`] — a byte, not the display name — stored under
//! [`PLUGIN_ID`](crate::PLUGIN_ID) with `set_byte`/`get_byte`, the same calls
//! [`curse`](crate::events::curse) uses. That keeps it from colliding with another plugin's
//! data, lets it survive relogs and restarts, keeps two kinds built on the same vanilla type
//! apart, and means renaming a kind does not orphan the entities already in the world.
//!
//! This trait holds only what **every** kind shares: a base type, an identity, a name, the
//! hook that customises a spawned entity, and [`spawn`](Self::spawn) itself. What a kind is
//! for, and where it may appear, is the business of the builder that extends this one:
//! [`MobBuilder`](crate::entities::mobs::mob_builder::MobBuilder) adds a category and a layer
//! list, and a trader additionally describes itself through
//! [`VillagerBuilder`](crate::entities::villagers::villager_builder::VillagerBuilder) inside
//! its own `apply` — there is no second trait for it.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, LivingEntity};

use crate::PLUGIN_ID;

/// Key under which a custom entity's identity is stored.
///
/// One key for every kind, mobs and villagers alike: the [`id`](EntityBuilder::id) is what
/// tells them apart, so a second key would only split the same answer across two places.
const IDENTITY_KEY: &str = "entity";

/// The shape every custom entity shares, and the marker that identifies it.
///
/// Implemented once per kind, so its base type and its name stay together. Nothing here
/// registers anything with the host — the entity type is always a vanilla
/// [`EntityType`] — so adding a kind means adding a file and its submodule in `lib.rs`.
///
/// Everything a kind does to the spawned entity goes through [`apply`](Self::apply), never
/// through a fixed table in this trait: attributes, equipment, a profession, or anything else
/// the API grows.
pub trait EntityBuilder {
    /// The vanilla entity this kind is built on, and the type actually spawned.
    fn base() -> EntityType;

    /// This kind's stable numeric identifier.
    ///
    /// Stored on the entity as a byte, so it must be unique **across every kind the plugin
    /// ships**, mobs and villagers together, and must not change once players have one in the
    /// world: it is what tells this kind apart from another built on the same
    /// [`base`](Self::base), and several kinds sharing one base type is expected. The number
    /// space is therefore shared, and a villager takes the next free number like any mob.
    ///
    /// Unlike the display name, this is not shown to players, so renaming a kind leaves
    /// existing entities still recognised as themselves.
    fn id() -> u8;

    /// The name shown above the entity, as the component the host renders.
    ///
    /// A [`TextComponent`] rather than a `&'static str` so a kind can colour, style or
    /// localise its name without [`spawn`](Self::spawn) growing a second, richer hook.
    /// `TextComponent::text("Beniguma")` is the plain case.
    ///
    /// The component is owned because it is a host resource and cannot be copied: build a
    /// fresh one per call (`TextComponent::text(..)` is cheap) instead of trying to cache
    /// and hand out a shared one.
    fn name() -> TextComponent;

    /// Makes `entity` into this kind of entity.
    ///
    /// Runs inside [`spawn`](Self::spawn), after the entity exists and before anything can
    /// meet it, so this is where a kind sets its attributes and equips itself. The default
    /// changes nothing, leaving the base type exactly as vanilla spawns it.
    ///
    /// Set attributes through `set_attribute_base`, not the current value. The base is the
    /// template and the visible stat is derived from it, so setting a base is idempotent —
    /// repeated calls never compound. Raising `MaxHealth` also means raising `set_health` to
    /// match, or the entity arrives damaged.
    ///
    /// Equipment is per slot, so a kind equips only what it means to: a hand-only mob calls
    /// `set_equipment(EquipmentSlot::MainHand, ..)` and leaves the armour slots empty, and a
    /// fully dressed one sets each slot it wants. Every other slot stays vanilla.
    fn apply(_entity: &LivingEntity) {}

    /// Spawns this entity in `world` at `position` and stamps it with its identity.
    ///
    /// Returns the spawned entity, which is the vanilla [`base`](Self::base) type carrying
    /// this kind's name, its customisations and its
    /// [`EntityBuilder::id`] marker — not a new entity type.
    ///
    /// Placement is not checked here: callers that spawn by rule should gate on the kind's
    /// own layer list first (see
    /// [`MobBuilder::spawns_in`](crate::entities::mobs::mob_builder::MobBuilder::spawns_in)),
    /// and callers placing an entity deliberately may skip the check.
    ///
    /// The name is set visible: without that a custom entity is indistinguishable from the
    /// vanilla one it is built on.
    fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        let entity = world.spawn_entity(Self::base(), position);

        entity.set_custom_name(Self::name());
        entity.set_custom_name_visible(true);

        if let Some(living) = entity.as_living() {
            Self::apply(&living);
        }

        mark(&entity, Self::id());

        entity
    }
}

/// Records that `entity` is a custom entity of kind `id`.
///
/// The identity is written by [`EntityBuilder::spawn`], which is the only caller. The byte is
/// signed, so a kind id has to fit in `i8`; a larger one is silently not written rather
/// than wrapped into another kind's id.
pub(crate) fn mark(entity: &Entity, id: u8) {
    if let Ok(id) = i8::try_from(id) {
        entity.set_byte(PLUGIN_ID, IDENTITY_KEY, id);
    }
}

/// Returns the identifier of the custom entity `entity` is, if it is one.
///
/// `None` means the entity was not spawned by this plugin — it is a vanilla spawn, a
/// spawner's, or a `/summon` — so a handler must treat it as such rather than assuming
/// every entity is custom.
///
/// The byte is stored signed, so a value above `127` comes back as `None`: a kind id that
/// large is more likely a mistake than an intent.
///
/// No handler reads an identity while `/abyss summon` is gone, but the marker is still
/// written on every spawn, so this stays the way to ask a spawned entity what it is.
#[expect(
    dead_code,
    reason = "the reader returns with the handler that needs it"
)]
#[must_use]
pub(crate) fn id_of(entity: &Entity) -> Option<u8> {
    entity
        .get_byte(PLUGIN_ID, IDENTITY_KEY)
        .and_then(|id| u8::try_from(id).ok())
}
