//! The Second Layer (Y 126–194) — the Forest of Temptation.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Second Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Second;

impl Layer for Second {
    fn depth(&self) -> Depth {
        Depth::Second
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.second.enabled)
    }
}

/// Configuration for the Second Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Forest of Temptation"`| Display name used in titles        |
/// | `gradient_start` | `"#5BA85B"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#2E8B57"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecondConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for SecondConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Forest of Temptation".to_owned(),
            gradient_start: "#5BA85B".to_owned(),
            gradient_end: "#2E8B57".to_owned(),
        }
    }
}
