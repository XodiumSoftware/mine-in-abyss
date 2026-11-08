//! The Fifth Layer (12000–13000 m) — the Sea of Corpses.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Fifth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fifth;

impl Layer for Fifth {
    fn depth(&self) -> Depth {
        Depth::Fifth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.layers)
    }
}
