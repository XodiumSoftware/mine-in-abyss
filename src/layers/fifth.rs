//! The Fifth Layer — the Sea of Corpses.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Fifth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fifth;

impl Layer for Fifth {
    fn id(&self) -> Section {
        Section::Fifth
    }
}
