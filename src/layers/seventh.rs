//! The Seventh Layer — the Final Maelstrom.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Seventh Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Seventh;

impl Layer for Seventh {
    fn id(&self) -> Section {
        Section::Seventh
    }
}
