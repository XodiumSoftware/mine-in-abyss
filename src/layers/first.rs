//! The First Layer (Y 195–319) — the top of the Abyss.
//!
//! Features that only apply near the surface belong here.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the First Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct First;

impl Layer for First {
    fn depth(&self) -> Depth {
        Depth::First
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.first.enabled)
    }
}

/// Configuration for the First Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Top of the Abyss"`| Display name used in titles        |
/// | `gradient_start` | `"#87CEEB"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#5BA85B"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FirstConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for FirstConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Top of the Abyss".to_owned(),
            gradient_start: "#87CEEB".to_owned(),
            gradient_end: "#5BA85B".to_owned(),
        }
    }
}
