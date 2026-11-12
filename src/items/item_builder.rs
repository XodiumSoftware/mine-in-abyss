//! The chainable [`ItemBuilder`] a custom item writes its stack with, and the identity that
//! is written onto it.
//!
//! An item cannot be registered with the host: `Item` is a closed enum of vanilla items and
//! the plugin API has no call to add one, so — exactly as with
//! [`EntityBuilder`](crate::entities::entity_builder) — a custom item here is **a vanilla item
//! wearing an identity**. What distinguishes it from a plain `minecraft:diamond` is a marker
//! in the stack's own persistent data, written once on creation and read by every handler
//! that needs to know what it is looking at.
//!
//! The marker is the id a kind passes to [`ItemBuilder::of`] — a byte, not the display name
//! — stored under [`PLUGIN_ID`](crate::PLUGIN_ID) with `set_byte`/`get_byte`, the same calls
//! [`curse`](crate::events::curse) uses for players and
//! [`entity_builder`](crate::entities::entity_builder) for entities. That keeps it from
//! colliding with another plugin's data, lets it survive restarts, keeps two items built on
//! the same vanilla item apart, and means renaming an item does not orphan the stacks already
//! in the world.
//!
//! There is deliberately no trait here. A kind is a constructor — an inherent function that
//! returns a built [`ItemStack`], as `Orthcoin::stack` does — because nothing is generic over
//! item kinds: there is no `item_kinds()` catalogue to erase them into (unlike
//! [`mob_kinds`](crate::mob_kinds)), so a trait would add an import and an indirection without
//! giving a caller anything. It returns when a generic consumer does.

use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::PLUGIN_ID;

/// Key under which a custom item's identity is stored.
const IDENTITY_KEY: &str = "item";

/// A custom item's stack, built one property at a time.
///
/// `ItemStack` is a host resource whose setters return nothing, so they cannot be chained
/// directly. This type wraps one and offers the same writes as consuming methods, which lets
/// a kind write its whole stack as a single expression:
///
/// ```ignore
/// ItemBuilder::of(1, Item::GoldNugget)
///     .name(TextComponent::text("Orthcoin"))
///     .lore(vec![TextComponent::text("Currency of Orth.")])
///     .build()
/// ```
///
/// The identity is a parameter of [`of`](Self::of) rather than a method of its own, so a
/// builder cannot exist without one: a stack cannot accidentally be left unmarked, and there
/// is no way to claim an id twice. The count is *not* — a kind starts a stack of one, which is
/// what almost every custom item is; a kind that is genuinely a bundle sets its count through
/// [`apply`](Self::apply).
///
/// It adds no capability of its own — every method is one `ItemStack` call, applied in the
/// order the chain runs, so a later `name` overrides an earlier one exactly as two direct
/// calls would. A kind needing a setter this type has not grown yet reaches the live stack
/// through [`apply`](Self::apply), which takes a closure over it:
///
/// ```ignore
/// ItemBuilder::of(1, Item::GoldNugget)
///     .name(TextComponent::text("Orthcoin"))
///     .apply(|stack| stack.add_custom_enchantment(.., .., 1))
///     .build()
/// ```
///
/// A kind's chain ends at `build()`, so it never leaves the kind: a caller receives an
/// `ItemStack` with nothing left to set.
///
/// It is named `ItemBuilder` rather than `StackBuilder` to sit beside
/// [`EntityBuilder`](crate::entities::entity_builder): it is what a kind builds *with*, not a
/// thing a caller holds.
#[derive(Debug)]
pub(crate) struct ItemBuilder {
    stack: ItemStack,
    id: u8,
}

impl ItemBuilder {
    /// Starts a stack of one of `base`, claiming `id` as its identity.
    ///
    /// The marker is not written here — [`build`](Self::build) writes it last, after every
    /// other property, so the identity on the finished stack is the one this constructor was
    /// given.
    ///
    /// The count is one because a custom item is a single named thing — a coin, a relic — not a
    /// batch of a vanilla one; a kind that really does come in a bundle sets
    /// `set_count` through [`apply`](Self::apply).
    ///
    /// The byte is signed, so an id above `127` cannot be stored: see
    /// [`build`](Self::build) for what happens then.
    pub(crate) fn of(id: u8, base: Item) -> Self {
        Self {
            stack: ItemStack::of(base, 1),
            id,
        }
    }

    /// Sets the name shown on the stack.
    #[must_use]
    pub(crate) fn name(self, name: TextComponent) -> Self {
        self.stack.set_custom_name(Some(name));
        self
    }

    /// Replaces the stack's lore with `lines`.
    #[must_use]
    pub(crate) fn lore(self, lines: Vec<TextComponent>) -> Self {
        self.stack.set_lore(lines);
        self
    }

    /// Runs `extend` over the live stack, for anything the chain has not grown a method for.
    ///
    /// This is the escape hatch: enchantments, attribute modifiers, dye, a banner pattern — any
    /// setter on [`ItemStack`] the convenience methods do not cover. `extend` receives a
    /// `&mut ItemStack` so it can make the same calls it would on a stack it owned, and the
    /// builder is handed back so the chain continues:
    ///
    /// ```ignore
    /// ItemBuilder::of(1, Item::GoldNugget)
    ///     .name(TextComponent::text("Orthcoin"))
    ///     .apply(|stack| stack.add_custom_enchantment(.., .., 1))
    ///     .build()
    /// ```
    ///
    /// It runs where it sits in the chain, so anything it sets is overridable by a later call
    /// and overrides an earlier one — exactly like writing each call directly. Because it sees
    /// the stack before [`build`](Self::build) writes the identity marker, it cannot disturb
    /// which kind the finished stack claims to be.
    #[must_use]
    pub(crate) fn apply(mut self, extend: impl FnOnce(&mut ItemStack)) -> Self {
        extend(&mut self.stack);
        self
    }

    /// Returns the finished stack, stamped with the identity it was built with.
    ///
    /// Consumes the builder so the result cannot be modified through it afterwards — the
    /// caller owns the stack from here.
    #[must_use]
    pub(crate) fn build(self) -> ItemStack {
        mark(&self.stack, self.id);
        self.stack
    }
}

/// Records that `stack` is a custom item of kind `id`.
///
/// Written by [`ItemBuilder::build`], which is the only caller. The byte is signed, so a
/// kind id has to fit in `i8`; a larger one is silently not written rather than wrapped into
/// another kind's id.
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
