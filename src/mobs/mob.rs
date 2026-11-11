//! The trait a custom mob implements, and the identity it writes onto the entity.
//!
//! An entity cannot change type, so a mob spawned by [`Mob::spawn`] is a vanilla mob to the
//! server. The only thing that distinguishes it from a naturally spawned one is a marker in
//! its own persistent data, written once at spawn and read by every handler that needs to
//! know what it is looking at.
//!
//! The marker is [`Mob::id`] — a byte, not the display name — stored under
//! [`PLUGIN_ID`](crate::PLUGIN_ID) with `set_byte`/`get_byte`, the same calls
//! [`curse`](crate::events::curse) uses. That keeps it from colliding with another plugin's
//! data, lets it survive relogs and restarts, keeps two kinds built on the same vanilla type
//! apart, and means renaming a kind does not orphan the mobs already in the world.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, LivingEntity};

use crate::PLUGIN_ID;

/// Key under which a custom mob's identity is stored.
const IDENTITY_KEY: &str = "mob";

/// A kind of mob the plugin defines on top of a vanilla one.
///
/// Implemented once per mob kind, so its base type, its name and its stats stay together.
/// Nothing here registers anything with the host — the entity type is always a vanilla
/// [`EntityType`] — so adding a mob means adding a file and its submodule in `lib.rs`.
///
/// Everything a kind does to the spawned entity goes through [`Mob::apply`], never through a
/// fixed table in this trait: attributes, equipment, or anything else the API grows. A kind
/// that wears armour and carries a sword and one that only carries a sword are both just
/// different bodies for that method.
pub trait Mob {
    /// The vanilla mob this kind is built on, and the type actually spawned.
    fn base() -> EntityType;

    /// This kind's stable numeric identifier.
    ///
    /// Stored on the entity as a byte, so it must be unique among the kinds the plugin
    /// ships and must not change once players have mobs in the world: it is what tells this
    /// kind apart from another built on the same [`base`](Self::base), and several kinds
    /// sharing one base type is expected.
    ///
    /// Unlike the display name, this is not shown to players, so renaming a kind leaves
    /// existing mobs still recognised as themselves.
    fn id() -> u8;

    /// The name shown above the mob.
    fn name() -> &'static str;

    /// Makes `entity` into this kind of mob.
    ///
    /// Runs inside [`spawn`](Self::spawn), after the entity exists and before anything can
    /// meet it, so this is where a kind sets its attributes and equips itself. The default
    /// changes nothing, leaving the base type exactly as vanilla spawns it.
    ///
    /// Set attributes through `set_attribute_base`, not the current value. The base is the
    /// template and the visible stat is derived from it, so setting a base is idempotent —
    /// that is what lets [`Monsters`](crate::events::monsters::Monsters) apply the depth
    /// scale on top without the two compounding. Raising `MaxHealth` also means raising
    /// `set_health` to match, or the mob arrives damaged.
    ///
    /// Equipment is per slot, so a kind equips only what it means to: a hand-only mob calls
    /// `set_equipment(EquipmentSlot::MainHand, ..)` and leaves the armour slots empty, and a
    /// fully dressed one sets each slot it wants. Every other slot stays vanilla.
    fn apply(_entity: &LivingEntity) {}

    /// Spawns this mob in `world` at `position` and stamps it with its identity.
    ///
    /// Returns the spawned entity, which is the vanilla [`base`](Self::base) type carrying
    /// this kind's name, stats, equipment and [`Mob::id`] marker — not a new entity type.
    ///
    /// The name is set visible: without that a custom mob is indistinguishable from the
    /// vanilla one it is built on.
    fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        let entity = world.spawn_entity(Self::base(), position);

        entity.set_custom_name(TextComponent::text(Self::name()));
        entity.set_custom_name_visible(true);

        if let Some(living) = entity.as_living() {
            Self::apply(&living);
        }

        mark(&entity, Self::id());

        entity
    }
}

/// Records that `entity` is a custom mob of kind `id`.
///
/// The identity is written by [`Mob::spawn`], which is the only caller. The byte is signed,
/// so a kind id has to fit in `i8`; a larger one is silently not written rather than
/// wrapped into another kind's id.
pub fn mark(entity: &Entity, id: u8) {
    if let Ok(id) = i8::try_from(id) {
        entity.set_byte(PLUGIN_ID, IDENTITY_KEY, id);
    }
}

/// Returns the identifier of the custom mob `entity` is, if it is one.
///
/// `None` means the entity was not spawned by this plugin — it is a vanilla spawn, a
/// spawner's, or a `/summon` — so a handler must treat it as such rather than assuming
/// every mob is custom.
///
/// The byte is stored signed, so a value above `127` comes back as `None`: a kind id that
/// large is more likely a mistake than an intent.
#[must_use]
pub fn id_of(entity: &Entity) -> Option<u8> {
    entity
        .get_byte(PLUGIN_ID, IDENTITY_KEY)
        .and_then(|id| u8::try_from(id).ok())
}

/// Returns whether `entity` is the custom mob kind `id`.
#[must_use]
pub fn is(entity: &Entity, id: u8) -> bool {
    id_of(entity).is_some_and(|found| found == id)
}
