//! The First Layer (0–1350 m) — the top of the Abyss.
//!
//! Features that only apply near the surface belong here. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the First Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct First;

impl Layer for First {
    fn depth(&self) -> Depth {
        Depth::First
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.is_enabled(Depth::First))
    }
}
