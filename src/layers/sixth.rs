//! The Sixth Layer — the Capital of the Unreturned.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Sixth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Sixth;

impl Layer for Sixth {
    fn id(&self) -> Section {
        Section::Sixth
    }
}
