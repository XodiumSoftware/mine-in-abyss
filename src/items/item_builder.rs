//! The trait a custom item implements, and the identity it writes onto the stack.
//!
//! An item cannot be registered with the host: `Item` is a closed enum of vanilla items and
//! the plugin API has no call to add one, so — exactly as with [`MobBuilder`](crate::mobs) — a
//! custom item here is **a vanilla item wearing an identity**. What distinguishes it from a
//! plain `minecraft:diamond` is a marker in the stack's own persistent data, written once
//! on creation and read by every handler that needs to know what it is looking at.
//!
//! The marker is [`ItemBuilder::id`] — a byte, not the display name — stored under
//! [`PLUGIN_ID`](crate::PLUGIN_ID) with `set_byte`/`get_byte`, the same calls
//! [`curse`](crate::events::curse) uses for players and [`entity_builder`](crate::entities::mobs::mob_builder) for
//! entities. That keeps it from colliding with another plugin's data, lets it survive
//! restarts, keeps two items built on the same vanilla item apart, and means renaming an
//! item does not orphan the stacks already in the world.

use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::PLUGIN_ID;

/// Key under which a custom item's identity is stored.
const IDENTITY_KEY: &str = "item";

/// A kind of item the plugin defines on top of a vanilla one.
///
/// Implemented once per item kind, so its base item and its name stay together. Nothing
/// here registers anything with the host — the base is always a vanilla [`Item`] — so
/// adding an item means adding a file and its submodule in `lib.rs`.
///
/// Everything a kind does to the created stack goes through [`ItemBuilder::apply`], never through
/// a fixed table in this trait: the lore, enchantments, or anything else the API grows. An
/// item that is only a renamed stack and one that carries lore and enchantments are both
/// just different bodies for that method.
pub(crate) trait ItemBuilder {
    /// The vanilla item this kind is built on, and the item actually created.
    fn base() -> Item;

    /// This kind's stable numeric identifier.
    ///
    /// Stored on the stack as a byte, so it must be unique among the items the plugin ships
    /// and must not change once players hold stacks of it: it is what tells this kind apart
    /// from another built on the same [`base`](Self::base), and several items sharing one
    /// base item is expected.
    ///
    /// Unlike the display name, this is not shown to players, so renaming an item leaves
    /// existing stacks still recognised as themselves.
    fn id() -> u8;

    /// The name shown on the stack, as the component the host renders.
    ///
    /// A [`TextComponent`] rather than a `&'static str` so a kind can colour or style its
    /// name. Build a fresh one per call (`TextComponent::text(..)` is cheap): it is a host
    /// resource and cannot be copied.
    fn name() -> TextComponent;

    /// Makes `stack` into this kind of item.
    ///
    /// Runs inside [`create`](Self::create), after the stack exists and before anything can
    /// hold it, so this is where a kind sets its lore or enchants itself. The default
    /// changes nothing, leaving the base item exactly as vanilla makes it.
    ///
    /// The name is set by [`create`](Self::create) from [`name`](Self::name) rather than
    /// here, because every kind has one and it is what makes a custom item visible at all.
    fn apply(_stack: &ItemStack) {}

    /// Creates a stack of `count` of this kind's base item, stamped with its identity.
    ///
    /// Returns the stack, which is the vanilla [`base`](Self::base) item carrying this
    /// kind's name, its customisations and its [`id`](Self::id) marker — not a new item
    /// type.
    fn create(count: u8) -> ItemStack {
        let stack = ItemStack::of(Self::base(), count);

        stack.set_custom_name(Some(Self::name()));
        Self::apply(&stack);
        mark(&stack, Self::id());

        stack
    }
}

/// Records that `stack` is a custom item of kind `id`.
///
/// The identity is written by [`ItemBuilder::create`], which is the only caller. The byte is
/// signed, so a kind id has to fit in `i8`; a larger one is silently not written rather
/// than wrapped into another kind's id.
pub(crate) fn mark(stack: &ItemStack, id: u8) {
    if let Ok(id) = i8::try_from(id) {
        stack.set_byte(PLUGIN_ID, IDENTITY_KEY, id);
    }
}

/// Returns the identifier of the custom item `stack` is, if it is one.
///
/// `None` means the stack was not created by this plugin — it is a vanilla item, a
/// loot-table result or a `/give` — so a handler must treat it as such rather than assuming
/// every stack is custom.
///
/// The byte is stored signed, so a value above `127` comes back as `None`: an id that large
/// is more likely a mistake than an intent.
#[must_use]
pub(crate) fn id_of(stack: &ItemStack) -> Option<u8> {
    stack
        .get_byte(PLUGIN_ID, IDENTITY_KEY)
        .and_then(|id| u8::try_from(id).ok())
}
