//! The Second Layer — the Forest of Temptation.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Second Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Second;

impl Layer for Second {
    fn id(&self) -> Section {
        Section::Second
    }

    fn name(&self) -> &'static str {
        "The Forest of Temptation"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x5B,
                g: 0xA8,
                b: 0x5B,
            },
            RgbColor {
                r: 0x2E,
                g: 0x8B,
                b: 0x57,
            },
        ]
    }
}
