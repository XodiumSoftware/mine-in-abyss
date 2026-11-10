//! The Third Layer — the Great Fault.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;

/// Stub for the Third Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Third;

impl Layer for Third {
    fn id(&self) -> Section {
        Section::Third
    }
}
