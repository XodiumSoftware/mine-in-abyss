//! The Seventh Layer — the Final Maelstrom.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Seventh Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Seventh;

impl Layer for Seventh {
    fn id(&self) -> Section {
        Section::Seventh
    }

    fn name(&self) -> &'static str {
        "The Final Maelstrom"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x1A,
                g: 0x17,
                b: 0x28,
            },
            RgbColor {
                r: 0x6B,
                g: 0x00,
                b: 0x00,
            },
        ]
    }
}
