//! The chainable [`EntityBuilder`] a custom entity is spawned with, and the identity that is
//! written onto it.
//!
//! Minecraft's entity types cannot be extended: an entity spawned through this plugin is a
//! vanilla type to the server. The only thing that distinguishes it from a naturally spawned
//! one is a marker in its own persistent data, written once at spawn and read by every handler
//! that needs to know what it is looking at.
//!
//! The marker is the id a caller passes to [`EntityBuilder::of`] — a byte, not the display name
//! — stored under [`PLUGIN_ID`](crate::PLUGIN_ID) with `set_byte`/`get_byte`, the same calls
//! [`curse`](crate::events::curse) uses for players and
//! [`item_builder`](crate::items::item_builder) for items. That keeps it from colliding with
//! another plugin's data, lets it survive relogs and restarts, keeps two kinds built on the
//! same vanilla type apart, and means renaming a kind does not orphan the entities already in
//! the world.
//!
//! There is deliberately no trait here. A kind is a constructor — an inherent function that
//! returns a built [`Entity`], as a mob's own `spawn` does — because a trait only earns its keep
//! when something is *generic* over it, and this builder gives a kind exactly what it needs
//! without one. It returns when a generic over entity kinds appears.
//!
//! # What this builder does and does not decide
//!
//! An entity's identity, its base type and its name are stated here, by whoever spawns it.
//! What a kind is *for* — a mob's category and layers, a trader's profession — is not the
//! builder's business: a kind describes those itself after it is spawned, so a builder never
//! has to know about [`MobCategory`](crate::entities::mobs::mob_category::MobCategory) or a
//! profession.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::Entity;

use crate::PLUGIN_ID;

/// Key under which a custom entity's identity is stored.
///
/// One key for every kind, mobs and traders alike: the id is what tells them apart, so a
/// second key would only split the same answer across two places.
const IDENTITY_KEY: &str = "entity";

/// The closure [`EntityBuilder::apply`] stores, run against the spawned entity.
type Extend = Box<dyn FnOnce(&Entity)>;

/// A custom entity, built one property at a time, then spawned and stamped.
///
/// `Entity` is a host resource whose setters return nothing, so they cannot be chained
/// directly. This type wraps the identity, the base type and the name and offers the same
/// writes as consuming methods, which lets a caller write the whole spawn as one expression:
///
/// ```ignore
/// EntityBuilder::of(1, EntityType::Wolf)
///     .name(TextComponent::text("Beniguma"))
///     .apply(|entity| entity.set_health(16.0))
///     .build(&world, position)
/// ```
///
/// The identity is a parameter of [`of`](Self::of) rather than a method of its own, so a
/// builder cannot exist without one: an entity cannot accidentally be spawned unmarked, and
/// there is no way to claim an id twice. The base type is the other required answer, for the
/// same reason — there is no sensible entity without a type to spawn.
///
/// It adds no capability of its own — every method is one `Entity` call, applied in the order
/// the chain runs, so a later `name` overrides an earlier one exactly as two direct calls
/// would. A caller needing a setter it has not grown yet reaches the live entity through
/// [`apply`](Self::apply).
///
/// The chain ends at [`build`](Self::build), which spawns the entity and returns it with the
/// identity marker already written, so a caller receives an `Entity` with nothing left to
/// stamp.
///
/// It is named `EntityBuilder` to sit beside
/// [`ItemBuilder`](crate::items::item_builder::ItemBuilder): both are what a kind builds *with*,
/// not things a caller holds.
pub(crate) struct EntityBuilder {
    id: u8,
    base: EntityType,
    name: Option<TextComponent>,
    extend: Option<Extend>,
}

impl EntityBuilder {
    /// Starts a spawn of `base`, claiming `id` as its identity.
    ///
    /// The marker is not written here — [`build`](Self::build) writes it last, after every
    /// other property, so the identity on the spawned entity is the one this constructor was
    /// given.
    ///
    /// The byte is signed, so an id above `127` cannot be stored: see
    /// [`build`](Self::build) for what happens then.
    pub(crate) fn of(id: u8, base: EntityType) -> Self {
        Self {
            id,
            base,
            name: None,
            extend: None,
        }
    }

    /// Sets the name shown above the entity.
    ///
    /// A [`TextComponent`], so a kind can colour, style or localise it — the same idea as
    /// [`ItemBuilder::name`](crate::items::item_builder::ItemBuilder::name).
    ///
    /// The name is made visible by [`build`](Self::build): without that a custom entity is
    /// indistinguishable from the vanilla one it is built on, which is the whole point of
    /// naming it.
    #[must_use]
    pub(crate) fn name(mut self, name: TextComponent) -> Self {
        self.name = Some(name);
        self
    }

    /// Runs `extend` over the spawned entity, once it exists.
    ///
    /// The escape hatch, matching
    /// [`ItemBuilder::apply`](crate::items::item_builder::ItemBuilder::apply): attributes and
    /// equipment are set here, and it runs before [`build`](Self::build) stamps the identity,
    /// so it cannot disturb which kind the entity claims to be.
    ///
    /// `extend` is handed the spawned [`Entity`], not its `LivingEntity`, so a kind that wants
    /// a specialised wrapper downcasts it itself with `EntityCastExt::cast`:
    ///
    /// ```ignore
    /// .apply(|entity| {
    ///     let Some(villager) = entity.cast::<Villager>() else { return };
    ///     villager.set_profession(VillagerProfession::Librarian);
    /// })
    /// ```
    ///
    /// Set attributes through `set_attribute_base`, not the current value. The base is the
    /// template and the visible stat is derived from it, so setting a base is idempotent —
    /// repeated calls never compound. Raising `MaxHealth` also means raising `set_health` to
    /// match, or the entity arrives damaged.
    ///
    /// Equipment is per slot, so a kind equips only what it means to: a hand-only mob calls
    /// `set_equipment(EquipmentSlot::MainHand, ..)` and leaves the armour slots empty, and a
    /// fully dressed one sets each slot it wants. Every other slot stays vanilla.
    #[must_use]
    pub(crate) fn apply(mut self, extend: impl FnOnce(&Entity) + 'static) -> Self {
        self.extend = Some(Box::new(extend));
        self
    }

    /// Spawns the entity in `world` at `position`, stamped with the identity it was built
    /// with, and returns it.
    ///
    /// Consumes the builder so the result cannot be modified through it afterwards — the
    /// caller owns the entity from here.
    ///
    /// Returns the vanilla [`base`](Self::of) type carrying the name and the customisations,
    /// and the marker — not a new entity type.
    ///
    /// Placement is not checked here: callers that spawn by rule should gate on the kind's own
    /// layer list first, and callers placing an entity deliberately may skip the check.
    #[must_use]
    pub(crate) fn build(self, world: &World, position: (f64, f64, f64)) -> Entity {
        let entity = world.spawn_entity(self.base, position);

        if let Some(name) = self.name {
            entity.set_custom_name(name);
            entity.set_custom_name_visible(true);
        }

        if let Some(extend) = self.extend {
            extend(&entity);
        }

        mark(&entity, self.id);

        entity
    }
}

/// Records that `entity` is a custom entity of kind `id`.
///
/// Written by [`EntityBuilder::build`], which is the only caller. The byte is signed, so a kind
/// id has to fit in `i8`; a larger one is silently not written rather than wrapped into another
/// kind's id.
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
/// No handler reads an identity yet: the marker is written on every spawn, and nothing
/// currently needs to ask what a spawned entity is, so this waits for the first such handler.
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
