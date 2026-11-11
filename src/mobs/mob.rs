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
use strum::{EnumIter, IntoStaticStr};

use crate::PLUGIN_ID;
use crate::layer::Layer;

/// Key under which a custom mob's identity is stored.
const IDENTITY_KEY: &str = "mob";

/// How a mob kind behaves towards players.
///
/// A separate type from [`EntityType`], which only says *what shape* a mob is: a wolf and a
/// zombie are both spawned as vanilla types, but one is [`Category::Passive`] and the other
/// [`Category::Hostile`]. The category is what a spawn rule, a difficulty tweak or a
/// bestiary entry reads when the vanilla type is not the question.
///
/// Its `id()` is a fixed `snake_case` spelling rather than the variant name, so `"passive"`
/// stays readable in a command argument, a data file or a log line, the same way
/// [`Layer::id`](crate::layer::Layer) does it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Category {
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

impl Category {
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
}

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

    /// The name shown above the mob, as the component the host renders.
    ///
    /// A [`TextComponent`] rather than a `&'static str` so a kind can colour, style or
    /// localise its name without [`spawn`](Self::spawn) growing a second, richer hook.
    /// `TextComponent::text("Beniguma")` is the plain case.
    ///
    /// The component is owned because it is a host resource and cannot be copied: build a
    /// fresh one per call (`TextComponent::text(..)` is cheap) instead of trying to cache
    /// and hand out a shared one.
    fn name() -> TextComponent;

    /// How this kind behaves towards players.
    ///
    /// Not derived from [`base`](Self::base): the vanilla type says what the mob looks like
    /// and how the host paths it, while the category says what it is for. They usually agree
    /// and are allowed to disagree — a kind built on a wolf can be [`Category::Hostile`] if
    /// that is what it is meant to be.
    ///
    /// The default is [`Category::Passive`], the least aggressive reading: a kind that has
    /// not said otherwise is not silently treated as something that hunts players.
    fn category() -> Category {
        Category::default()
    }

    /// The layers this kind is allowed to spawn in.
    ///
    /// A slice rather than a single [`Layer`] so a kind can live at several depths, and
    /// rather than a predicate so the answer is data a spawn rule can read without
    /// running gameplay code. `&[Layer::First]` for a surface mob, `Layer::ALL` for one
    /// that belongs everywhere.
    ///
    /// The default is empty: a kind that has not said where it lives spawns nowhere, so a
    /// new mob cannot appear somewhere it was never meant to until it states its layers.
    /// This is a constraint on spawning only — [`spawn`](Self::spawn) itself does not
    /// consult it, because an operator placing a mob deliberately should not be blocked.
    fn layers() -> &'static [Layer] {
        &[]
    }

    /// Returns whether this kind may spawn in `layer`.
    fn spawns_in(layer: Layer) -> bool {
        Self::layers().contains(&layer)
    }

    /// Makes `entity` into this kind of mob.
    ///
    /// Runs inside [`spawn`](Self::spawn), after the entity exists and before anything can
    /// meet it, so this is where a kind sets its attributes and equips itself. The default
    /// changes nothing, leaving the base type exactly as vanilla spawns it.
    ///
    /// Set attributes through `set_attribute_base`, not the current value. The base is the
    /// template and the visible stat is derived from it, so setting a base is idempotent —
    /// repeated calls never compound. Raising `MaxHealth` also means raising `set_health` to
    /// match, or the mob arrives damaged.
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
    /// Placement is not checked here: callers that spawn by rule should gate on
    /// [`spawns_in`](Self::spawns_in) first, and callers placing a mob deliberately may
    /// skip the check.
    ///
    /// The name is set visible: without that a custom mob is indistinguishable from the
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
