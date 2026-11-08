//! `mine-in-abyss` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in
//! Rust that recreates ***Made in Abyss*** inside Minecraft.
//!
//! This crate is currently a **stub**: the plugin boots, loads its config, and exposes the module
//! registration pipeline that future content (the Abyss layers, relics, curses, delvers, ...) will
//! plug into.
//!
//! See [`GUIDE.md`](../GUIDE.md) for installation and configuration.
#![allow(clippy::must_use_candidate)]

mod config;
mod modules {
    pub mod mechanics {
        pub mod mechanic;
    }
}

pub use config::*;
pub use modules::*;

use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

/// The plugin identifier used for namespaced ids and permission nodes.
///
/// Must be a valid Minecraft identifier, so the crate name (`mine-in-abyss`)
/// is flattened to its underscore form.
pub const PLUGIN_ID: &str = "mine_in_abyss";

/// Entry point of the plugin.
pub struct MineInAbyss {}

impl Plugin for MineInAbyss {
    fn new() -> Self {
        Self {}
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: PLUGIN_ID.into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS")
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            dependencies: vec![],
            permissions: vec![
                pumpkin_plugin_api::permissions::FS_READ_DATA.into(),
                pumpkin_plugin_api::permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let _config = ConfigManager::load(&context);

        // TODO: register mechanics once real content exists.
        info!("Mine in Abyss loaded.");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Mine in Abyss unloaded.");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(MineInAbyss);
