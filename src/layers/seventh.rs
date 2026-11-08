//! The Seventh Layer (Y -64 to -51) — the Final Maelstrom.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Seventh Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Seventh;

impl Layer for Seventh {
    fn depth(&self) -> Depth {
        Depth::Seventh
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.seventh.enabled)
    }
}

/// Configuration for the Seventh Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Final Maelstrom"`| Display name used in titles        |
/// | `gradient_start` | `"#1A1728"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#6B0000"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SeventhConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for SeventhConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Final Maelstrom".to_owned(),
            gradient_start: "#1A1728".to_owned(),
            gradient_end: "#6B0000".to_owned(),
        }
    }
}
