//! The First Layer — the top of the Abyss.
//!
//! Features that only apply near the surface belong here.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the First Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct First;

impl Layer for First {
    fn id(&self) -> Section {
        Section::First
    }
}
