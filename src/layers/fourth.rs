//! The Fourth Layer (7000–12000 m) — the Goblets of Giants.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Fourth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fourth;

impl Layer for Fourth {
    fn depth(&self) -> Depth {
        Depth::Fourth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.is_enabled(Depth::Fourth))
    }
}
