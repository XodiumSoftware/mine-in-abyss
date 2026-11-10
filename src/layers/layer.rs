//! Layer system for `mine-in-abyss`.

use crate::section::Section;

/// A trait representing a plugin layer.
///
/// Display data — the name and palette — lives on [`Section`], so a layer only needs to
/// say which section it is. Behaviour that is genuinely specific to one layer belongs in
/// its `impl` block.
pub trait Layer {
    /// Returns the section of the Abyss this layer lives in.
    fn id(&self) -> Section;
}
