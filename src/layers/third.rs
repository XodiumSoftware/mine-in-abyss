//! The Third Layer (Y 66–125) — the Great Fault.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Third Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Third;

impl Layer for Third {
    fn depth(&self) -> Depth {
        Depth::Third
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.third.enabled)
    }
}

/// Configuration for the Third Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Great Fault"`| Display name used in titles        |
/// | `gradient_start` | `"#8B7355"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#6B5B4A"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThirdConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for ThirdConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Great Fault".to_owned(),
            gradient_start: "#8B7355".to_owned(),
            gradient_end: "#6B5B4A".to_owned(),
        }
    }
}
