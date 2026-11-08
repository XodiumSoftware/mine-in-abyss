//! The First Layer (Y 195–319) — the top of the Abyss.
//!
//! Features that only apply near the surface belong here.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the First Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct First;

impl Layer for First {
    fn id(&self) -> Section {
        Section::First
    }

    fn enabled(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "The Top of the Abyss"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x87,
                g: 0xCE,
                b: 0xEB,
            },
            RgbColor {
                r: 0x5B,
                g: 0xA8,
                b: 0x5B,
            },
        ]
    }
}
