//! The Seventh Layer (15500 m and below) — the Final Maelstrom.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Seventh Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Seventh;

impl Layer for Seventh {
    fn depth(&self) -> Depth {
        Depth::Seventh
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.is_enabled(Depth::Seventh))
    }
}
