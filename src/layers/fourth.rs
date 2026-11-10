//! The Fourth Layer — the Goblets of Giants.
//!
//! Features that only apply here belong in this module.

use crate::layers::layer::Layer;
use crate::section::Section;
use pumpkin_plugin_api::common::RgbColor;

/// Stub for the Fourth Layer. Gameplay content is not implemented yet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fourth;

impl Layer for Fourth {
    fn id(&self) -> Section {
        Section::Fourth
    }

    fn enabled(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "The Goblets of Giants"
    }

    fn color(&self) -> [RgbColor; 2] {
        [
            RgbColor {
                r: 0x6B,
                g: 0x5B,
                b: 0x4A,
            },
            RgbColor {
                r: 0x4A,
                g: 0x3F,
                b: 0x5C,
            },
        ]
    }
}
