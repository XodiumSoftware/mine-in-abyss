//! The Third Layer (Y 66–125) — the Great Fault.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Third Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Third;

impl Layer for Third {
    fn name(&self) -> &str {
        "The Great Fault"
    }

    fn world(&self) -> &str {
        "mine_in_abyss_third"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x8B,
                g: 0x73,
                b: 0x55,
            },
            RgbColor {
                r: 0x6B,
                g: 0x5B,
                b: 0x4A,
            },
        ]
    }

    fn enabled(&self) -> bool {
        false
    }
}
