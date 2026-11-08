//! Layer system for `mine-in-abyss`.

use pumpkin_plugin_api::common::RgbColor;

/// A trait representing a plugin layer.
pub trait Layer {
    /// Returns this layer's display name.
    fn name(&self) -> &str;

    /// Returns the name of the Minecraft world that holds this layer.
    fn world(&self) -> &str;

    /// Returns the gradient stop colours of this layer's palette.
    fn color(&self) -> [RgbColor; 2];

    /// Returns `true` if this layer is enabled, `false` otherwise.
    fn enabled(&self) -> bool;
}
