//! Configuration management system.
//!
//! The plugin configuration lives in the plugin's data folder:
//!
//! | File          | Purpose                              |
//! |---------------|--------------------------------------|
//! | `config.toml` | Master toggles for the layers group  |
//!
//! Each file is merged layered onto its typed defaults using the
//! [`config`](https://crates.io/crates/config) crate, so missing keys fall back
//! to defaults and extra keys in user files are preserved when re-saved.

use config::{File, FileFormat};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use tracing::error;

use crate::world_manager::WorldsConfig;

thread_local! {
    static CONFIG: RefCell<Option<PluginConfig>> = const { RefCell::new(None) };
}

/// Master group toggles stored in `config.toml`.
///
/// Each group will gain its own `<group>.toml` file as layers are added.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LayerToggles {
    /// Master toggle for all Abyss layers.
    pub layers: bool,
    /// Per-layer world creation settings.
    pub worlds: WorldsConfig,
}

impl Default for LayerToggles {
    fn default() -> Self {
        Self {
            layers: true,
            worlds: WorldsConfig::default(),
        }
    }
}

/// Top-level plugin configuration.
///
/// The `layers` table is read from `config.toml` and controls whether the
/// layers group is registered on load.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PluginConfig {
    /// Master toggles for the layers group (from `config.toml`).
    pub layers: LayerToggles,
}

/// Loads and provides access to the plugin configuration.
#[derive(Debug, Clone, Copy)]
pub struct ConfigManager;

impl ConfigManager {
    /// Loads `config.toml`, stores the merged result globally, and persists the
    /// merged file back to disk.
    ///
    /// Call this once in `Plugin::on_load` after all layers are ready.
    pub fn load(context: &Context) -> PluginConfig {
        let data_folder = PathBuf::from(context.get_data_folder());
        let config: PluginConfig = load_section(&data_folder, "config.toml");
        CONFIG.with(|c| *c.borrow_mut() = Some(config.clone()));
        config
    }

    /// Returns the loaded configuration, if any.
    #[must_use]
    pub fn get() -> Option<PluginConfig> {
        CONFIG.with(|c| c.borrow().clone())
    }
}

/// Loads a single TOML section file from `data_folder`, merges it over
/// `T::default()`, writes the merged result back to disk, and returns the
/// typed value. Falls back to `T::default()` on any error.
fn load_section<T>(data_folder: &Path, file_name: &str) -> T
where
    T: Default + Serialize + for<'de> Deserialize<'de>,
{
    let path = data_folder.join(file_name);

    let defaults_toml = toml::to_string(&T::default()).unwrap_or_else(|e| {
        error!("Failed to serialize defaults for {file_name}: {e}");
        String::new()
    });

    let builder = config::Config::builder()
        .add_source(File::from_str(&defaults_toml, FileFormat::Toml))
        .add_source(File::from(path.clone()).required(false));

    let cfg = match builder.build() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build config for {file_name}: {e}");
            return T::default();
        }
    };

    let merged: Value = match cfg.try_deserialize() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to deserialize merged config for {file_name}: {e}");
            return T::default();
        }
    };

    let value: T = match merged.clone().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse merged config for {file_name}: {e}");
            T::default()
        }
    };

    if let Some(parent) = path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create config directory for {file_name}: {e}");
    }

    if let Err(e) = fs::write(&path, toml::to_string_pretty(&merged).unwrap_or_default()) {
        error!("Failed to write config file {file_name}: {e}");
    }

    value
}
