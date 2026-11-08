//! The Fourth Layer (Y 6–65) — the Goblets of Giants.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Fourth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fourth;

impl Layer for Fourth {
    fn depth(&self) -> Depth {
        Depth::Fourth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.fourth.enabled)
    }
}

/// Configuration for the Fourth Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Goblets of Giants"`| Display name used in titles        |
/// | `gradient_start` | `"#6B5B4A"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#4A3F5C"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FourthConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for FourthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Goblets of Giants".to_owned(),
            gradient_start: "#6B5B4A".to_owned(),
            gradient_end: "#4A3F5C".to_owned(),
        }
    }
}
