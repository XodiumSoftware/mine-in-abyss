//! The Fifth Layer — the Sea of Corpses.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Fifth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fifth;

impl Layer for Fifth {
    fn id(&self) -> Section {
        Section::Fifth
    }

    fn name(&self) -> &'static str {
        "The Sea of Corpses"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x4A,
                g: 0x3F,
                b: 0x5C,
            },
            RgbColor {
                r: 0x2E,
                g: 0x2A,
                b: 0x45,
            },
        ]
    }
}
