//! The Third Layer (Y 66–125) — the Great Fault.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Third Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Third;

impl Layer for Third {
    fn id(&self) -> Section {
        Section::Third
    }

    fn enabled(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "The Great Fault"
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
}
