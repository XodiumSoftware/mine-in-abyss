//! Spawning a custom entity, and the identity written onto it.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::Entity;

use crate::PLUGIN_ID;

/// One key for every kind: the id distinguishes them.
const IDENTITY_KEY: &str = "entity";

/// The closure [`EntityBuilder::apply`] stores, run against the spawned entity.
type Extend = Box<dyn FnOnce(&Entity)>;

/// Builds a custom entity property by property, then spawns and stamps it.
///
/// `Entity`'s setters return nothing, so they cannot be chained; this wraps the ones a kind
/// needs. Nothing is added beyond those calls, so a setter it lacks can be reached through
/// [`apply`](Self::apply).
pub(crate) struct EntityBuilder {
    id: u8,
    base: EntityType,
    name: Option<TextComponent>,
    extend: Option<Extend>,
}

impl EntityBuilder {
    /// Starts a spawn of `base`, claiming `id` as its identity.
    ///
    /// The identity is a parameter rather than a method so a builder cannot exist without one.
    pub(crate) fn of(id: u8, base: EntityType) -> Self {
        Self {
            id,
            base,
            name: None,
            extend: None,
        }
    }

    /// Sets the name shown above the entity.
    #[must_use]
    pub(crate) fn name(mut self, name: TextComponent) -> Self {
        self.name = Some(name);
        self
    }

    /// Runs `extend` over the spawned entity.
    ///
    /// Attributes and equipment are set here. Set attributes through `set_attribute_base`:
    /// the base is the template, so repeated calls never compound, and raising `MaxHealth`
    /// means raising `set_health` to match.
    #[must_use]
    pub(crate) fn apply(mut self, extend: impl FnOnce(&Entity) + 'static) -> Self {
        self.extend = Some(Box::new(extend));
        self
    }

    /// Spawns the entity in `world` at `position`, stamped with this builder's identity.
    ///
    /// Placement is not checked: a caller spawning by rule gates on the kind's layer list
    /// first, and a caller placing an entity deliberately may skip it.
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
/// The byte is signed, so an id above `127` is silently not written rather than wrapped onto
/// another kind.
pub(crate) fn mark(entity: &Entity, id: u8) {
    if let Ok(id) = i8::try_from(id) {
        entity.set_byte(PLUGIN_ID, IDENTITY_KEY, id);
    }
}

/// Returns the identifier of the custom entity `entity` is, if it is one.
///
/// `None` means this plugin did not spawn it, so a handler must not assume every entity is
/// custom.
#[expect(dead_code, reason = "no handler reads an identity yet")]
#[must_use]
pub(crate) fn id_of(entity: &Entity) -> Option<u8> {
    entity
        .get_byte(PLUGIN_ID, IDENTITY_KEY)
        .and_then(|id| u8::try_from(id).ok())
}
