//! Layer system for `mine-in-abyss`.

use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// A trait representing a plugin layer.
pub trait Layer {
    /// Returns the section of the Abyss this layer lives in.
    fn id(&self) -> Section;

    /// Returns this layer's display name.
    fn name(&self) -> &'static str;

    /// Returns the gradient stop colours of this layer's palette.
    fn color(&self) -> [RgbColor; 2];
}
