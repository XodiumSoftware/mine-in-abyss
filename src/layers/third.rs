//! The Third Layer (2600–7000 m) — the Great Fault.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Third Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Third;

impl Layer for Third {
    fn depth(&self) -> Depth {
        Depth::Third
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.is_enabled(Depth::Third))
    }
}
