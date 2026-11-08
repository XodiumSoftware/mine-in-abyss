//! The Sixth Layer (Y -50 to -25) — the Capital of the Unreturned.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Sixth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Sixth;

impl Layer for Sixth {
    fn name(&self) -> &str {
        "The Capital of the Unreturned"
    }

    fn world(&self) -> &str {
        "mine_in_abyss_sixth"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x2E,
                g: 0x2A,
                b: 0x45,
            },
            RgbColor {
                r: 0x1A,
                g: 0x17,
                b: 0x28,
            },
        ]
    }

    fn enabled(&self) -> bool {
        false
    }
}
