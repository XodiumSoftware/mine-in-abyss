//! The Second Layer (1350–2600 m) — the Forest of Temptation.
//!
//! Features that only apply here belong in this module. Populated in
//! [`LayersConfig`](crate::layers::layer::LayersConfig) as they are added.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};

/// Stub for the Second Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Second;

impl Layer for Second {
    fn depth(&self) -> Depth {
        Depth::Second
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.layers)
    }
}
