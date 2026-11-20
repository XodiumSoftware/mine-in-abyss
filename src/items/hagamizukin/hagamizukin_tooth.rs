//! The Hagamizukin Tooth.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Hagamizukin Tooth a Hagamizukin leaves behind, built on a bone.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct HagamizukinTooth;

impl HagamizukinTooth {
    /// Creates a stamped stack of one Hagamizukin Tooth.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone: one drop is one thing, so the stack starts at one and no chain
    /// method contradicts it.
    ///
    /// Nothing calls this yet — the drop table lists it but the host cannot be told to deliver
    /// it — so `mod items` carries `#[expect(dead_code)]` for the whole tree.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(34, Item::Bone)
            .name(TextComponent::text("Hagamizukin Tooth").color_rgb(rgb(0x0B, 0x66, 0x23)))
            .lore(vec![TextComponent::text(
                "Hagamizukin Tooth of a Hagamizukin.",
            )])
            .build()
    }
}
