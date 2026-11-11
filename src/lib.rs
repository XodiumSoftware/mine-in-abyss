//! `mine-in-abyss` is a [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft plugin written in
//! Rust that recreates ***Made in Abyss*** inside Minecraft.
//!
//! Each layer of the Abyss is its own Minecraft world, and players descend by falling
//! through the void of the layer above. Layers are [`Layer`] variants, which carry
//! their identifier, display name and palette; gameplay content for individual layers is
//! still a stub, but the Curse of the Abyss and layer transitions are implemented.
//!
//! See [`GUIDE.md`](../GUIDE.md) for installation.
#![allow(clippy::must_use_candidate)]

mod layer;

/// The `/abyss` command.
mod commands {
    pub mod abyss;
    pub mod command;
}

/// The layer worlds, and the machinery that keeps them usable.
///
/// A layer world is only ever as good as the three pieces around it: it has to be created
/// ([`setup`]), it has to generate empty until its authored map exists ([`generator`]),
/// and it has to share the surface's clock with the rest of the Abyss ([`clock`]).
mod world {
    pub mod clock;
    pub mod generator;
    pub mod setup;
}

/// Small helpers shared across the plugin, split by what each module owns.
///
/// The three `thread_local` records a transition needs are each a module of their own, so
/// the state and the functions that maintain it cannot drift apart. Every submodule is
/// re-exported here, so a caller writes `use crate::utils::{carry_fall, layer_of};` and
/// does not have to know which file a helper ended up in; reach for `utils::fall::…`
/// directly only when the grouping is what you are looking for.
mod utils {
    pub mod fall;
    pub mod general;
    pub mod teleport;
    pub mod text;
    pub mod transition;
    pub mod world;

    pub use fall::{carry_fall, clear_fall, resume_fall};
    pub use teleport::{arrived_by_teleport, note_teleport};
    pub use text::{ordinal, rgb, tell};
    pub use transition::{begin_transition, end_transition, ignores_transitions};
    pub use world::{layer_of, world_for};
}

/// Event handlers for `mine-in-abyss`.
mod events {
    pub mod bed_spawn;
    pub mod curse;
    pub mod event;
    pub mod fall_damage;
    pub mod layer_join;
    pub mod layer_leave;
    pub mod layer_respawn;
    pub mod layer_teleport;
    pub mod monsters;
}

pub use layer::*;
pub use world::setup::*;

use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use std::time::Instant;
use tracing::info;

use commands::command::Command as RegisterCommand;
use events::event::Event as RegisterEvent;

/// The plugin identifier used for namespaced ids and permission nodes.
///
/// Must be a valid Minecraft identifier, so the crate name (`mine-in-abyss`)
/// is flattened to its underscore form.
pub const PLUGIN_ID: &str = "mine_in_abyss";

/// Every layer, from the surface down.
///
/// This is the single source of truth for which layers exist: registration, world
/// creation and the entry title all read from it. Identical to [`Layer::ALL`].
pub const LAYERS: [Layer; 7] = Layer::ALL;

/// Returns the layer whose world is called `name`, if any.
pub(crate) fn layer_for_world(name: &str) -> Option<Layer> {
    LAYERS.into_iter().find(|layer| layer.world() == name)
}

/// Entry point of the plugin.
pub struct MineInAbyss {}

impl MineInAbyss {
    fn layers() {
        let started = Instant::now();

        info!(
            "Registered {} layer(s) in {:?}",
            LAYERS.len(),
            started.elapsed()
        );
    }

    fn events(context: &Context) {
        let events: [fn(&Context); 7] = [
            events::layer_join::LayerJoin::register,
            events::layer_leave::LayerLeave::register,
            events::fall_damage::FallDamage::register,
            events::layer_respawn::LayerRespawn::register,
            events::layer_teleport::LayerTeleport::register,
            events::monsters::Monsters::register,
            events::bed_spawn::BedSpawn::register,
        ];

        let started = Instant::now();

        for register in events {
            register(context);
        }

        info!(
            "Registered {} event(s) in {:?}",
            events.len(),
            started.elapsed()
        );
    }

    fn commands(context: &Context) {
        let commands: [fn(&Context); 1] = [commands::abyss::Abyss::register];

        let started = Instant::now();

        for register in commands {
            register(context);
        }

        info!(
            "Registered {} command(s) in {:?}",
            commands.len(),
            started.elapsed()
        );
    }

    fn worlds(context: &Context) {
        let started = Instant::now();

        Worlds::setup(context, &LAYERS);
        world::clock::schedule(context);

        info!(
            "World setup completed and world clocks are in sync in {:?}",
            started.elapsed()
        );
    }
}

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
            permissions: vec![],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        Self::layers();
        Self::events(&context);
        Self::commands(&context);
        Self::worlds(&context);

        info!("Mine In Abyss loaded. NICE TO CYA!");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Mine In Abyss unloaded. CYA NEXT TIME!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(MineInAbyss);
