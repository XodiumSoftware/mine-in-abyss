//! The Fourth Layer — the Goblets of Giants.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Fourth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fourth;

impl Layer for Fourth {
    fn id(&self) -> Section {
        Section::Fourth
    }
}
