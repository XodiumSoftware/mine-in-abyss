//! The Sixth Layer (Y -50 to -25) — the Capital of the Unreturned.
//!
//! Features that only apply here belong in this module.

use crate::config::ConfigManager;
use crate::layers::layer::{Depth, Layer};
use serde::{Deserialize, Serialize};

/// Stub for the Sixth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Sixth;

impl Layer for Sixth {
    fn depth(&self) -> Depth {
        Depth::Sixth
    }

    fn enabled(&self) -> bool {
        ConfigManager::get().is_some_and(|cm| cm.layers.config.sixth.enabled)
    }
}

/// Configuration for the Sixth Layer.
///
/// | Field            | Default              | Description                        |
/// | ---------------- | -------------------- | ---------------------------------- |
/// | `enabled`        | `false`              | Whether this layer is active       |
/// | `name`           | `"The Capital of the Unreturned"`| Display name used in titles        |
/// | `gradient_start` | `"#2E2A45"` | Gradient start colour (`#RRGGBB`)  |
/// | `gradient_end`   | `"#1A1728"` | Gradient end colour (`#RRGGBB`)    |
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SixthConfig {
    /// Whether this layer is active.
    pub enabled: bool,
    /// Display name; falls back to the built-in name when blank.
    pub name: String,
    /// Start colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_start: String,
    /// End colour of the title gradient, as a `#RRGGBB` hex string.
    pub gradient_end: String,
}

impl Default for SixthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "The Capital of the Unreturned".to_owned(),
            gradient_start: "#2E2A45".to_owned(),
            gradient_end: "#1A1728".to_owned(),
        }
    }
}
