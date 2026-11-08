//! The Fifth Layer (Y -24 to 5) — the Sea of Corpses.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Fifth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fifth;

impl Layer for Fifth {
    fn depth(&self) -> Depth {
        Depth::Fifth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.fifth.enabled)
    }
}

/// Configuration for the Fifth Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Sea of Corpses"`| Display name used in titles        |
/// | `gradient_start` | `"#4A3F5C"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#2E2A45"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FifthConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for FifthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Sea of Corpses".to_owned(),
            gradient_start: "#4A3F5C".to_owned(),
            gradient_end: "#2E2A45".to_owned(),
        }
    }
}
