//! Rendering values for the player-facing text the plugin builds.

use pumpkin_plugin_api::Player;
use pumpkin_plugin_api::common::{NamedColor, RgbColor};
use pumpkin_plugin_api::text::TextComponent;

/// Builds an [`RgbColor`] from its channels, so palettes can be written inline.
#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
    RgbColor { r, g, b }
}

/// Renders `number` with its English ordinal suffix (`1st`, `2nd`, `3rd`, `4th`, …).
///
/// Callers pass what they already have — a layer's [`number`](crate::Layer::number), for
/// instance — rather than the plugin storing a formatted ordinal anywhere, so a renamed
/// or reordered list cannot leave a stale suffix behind.
#[must_use]
pub fn ordinal(number: usize) -> String {
    let suffix = match number {
        11..=13 => "th",
        _ => match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{number}{suffix}")
}

/// Tells `player` something with a short message in the action bar.
///
/// The action bar rather than chat: these are answers to one interaction — a refused
/// spawn point, a layer with nothing beside it — and should not clutter the log. `color`
/// carries the meaning, red for a refusal and grey for a neutral explanation.
pub fn tell(player: &Player, message: &str, color: NamedColor) {
    player.show_actionbar(TextComponent::text(message).color_named(color));
}
