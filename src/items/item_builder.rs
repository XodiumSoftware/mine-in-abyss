//! Building a custom item's stack, and the identity written onto it.

use pumpkin_plugin_api::item::{Item, ItemStackExt};
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::PLUGIN_ID;

/// Key under which a custom item's identity is stored.
const IDENTITY_KEY: &str = "item";

/// Builds a custom item's stack property by property.
///
/// `ItemStack`'s setters return nothing, so they cannot be chained; this wraps the ones a
/// kind needs. Nothing is added beyond those calls, so a setter it lacks can be reached
/// through [`apply`](Self::apply).
#[derive(Debug)]
pub(crate) struct ItemBuilder {
    id: u8,
    stack: ItemStack,
}

impl ItemBuilder {
    /// Starts a stack of one of `base`, claiming `id` as its identity.
    ///
    /// The identity is a parameter so a stack cannot be left unmarked. The count is one
    /// because a custom item is a single named thing; a bundle sets its count in
    /// [`apply`](Self::apply).
    pub(crate) fn of(id: u8, base: Item) -> Self {
        Self {
            id,
            stack: ItemStack::of(base, 1),
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

    /// Runs `extend` over the live stack, for any setter the chain has not grown a method for.
    ///
    /// It runs where it sits in the chain, and before [`build`](Self::build) writes the
    /// identity, so it cannot disturb which kind the stack claims to be.
    #[must_use]
    #[expect(dead_code, reason = "no kind needs a setter the chain lacks yet")]
    pub(crate) fn apply(mut self, extend: impl FnOnce(&mut ItemStack)) -> Self {
        extend(&mut self.stack);
        self
    }

    /// Returns the finished stack, stamped with the identity it was built with.
    #[must_use]
    pub(crate) fn build(self) -> ItemStack {
        mark(&self.stack, self.id);
        self.stack
    }
}

/// Records that `stack` is a custom item of kind `id`.
///
/// The byte is signed, so an id above `127` is silently not written rather than wrapped onto
/// another kind.
pub(crate) fn mark(stack: &ItemStack, id: u8) {
    if let Ok(id) = i8::try_from(id) {
        stack.set_byte(PLUGIN_ID, IDENTITY_KEY, id);
    }
}

/// Returns the identifier of the custom item `stack` is, if it is one.
///
/// `None` means this plugin did not create it, so a handler must not assume every stack is
/// custom. Nothing reads it yet: `/abyss item` gives a stack away but never has one handed
/// back to it.
#[expect(dead_code, reason = "no handler reads an item identity yet")]
#[must_use]
pub(crate) fn id_of(stack: &ItemStack) -> Option<u8> {
    stack
        .get_byte(PLUGIN_ID, IDENTITY_KEY)
        .and_then(|id| u8::try_from(id).ok())
}
