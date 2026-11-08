//! The Sixth Layer (13000–15500 m) — the Capital of the Unreturned.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Sixth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Sixth;

impl Layer for Sixth {
    fn depth(&self) -> Depth {
        Depth::Sixth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.layers)
    }
}
