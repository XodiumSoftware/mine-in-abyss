//! The Second Layer — the Forest of Temptation.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Second Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Second;

impl Layer for Second {
    fn id(&self) -> Section {
        Section::Second
    }
}
