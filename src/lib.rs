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

/// The plugin's own mobs, built on top of the vanilla ones.
///
/// Minecraft's mob types cannot be extended, so a mob here is a vanilla entity wearing an
/// identity: a base type, a visible name, scaled attributes, and a marker in its own
/// persistent data. `mob_builder` is the trait a kind implements and where that marker is
/// written and read back; `docile::beniguma` is the worked example, filed in `docile`
/// because that is the category it declares.
mod mobs {
    pub mod mob_builder;

    /// Aerial mob kinds.
    pub mod aerial {
        pub mod buroguro;
        pub mod corpse_weeper;
        pub mod cyatoria;
        pub mod dosetori;
        pub mod giant_hammerbeak;
        pub mod hashofu;
        pub mod kagi_tachikiri;
        pub mod madokajack;
        pub mod onitsuchi;
        pub mod rohana;
        pub mod small_hammerbeak;
        pub mod splitjaw;
        pub mod spore_manta;
        pub mod squidfly;
        pub mod stinger;
    }

    /// Aquatic mob kinds.
    pub mod aquatic {
        pub mod demonfish;
        pub mod edusan;
        pub mod hagamizukin;
        pub mod hisomukuro;
        pub mod ikenosuke;
        pub mod kazura;
        pub mod makihige;
        pub mod spiralsquid;
    }

    /// Docile mob kinds.
    pub mod docile {
        pub mod beniguma;
        pub mod chimokami;
        pub mod daikonchu;
        pub mod hisoishi;
        pub mod rinsipede;
        pub mod rockwalker;
        pub mod spikewalker;
        pub mod titanjaw;
    }

    /// Hostile mob kinds.
    pub mod hostile {
        pub mod chikyuoebi;
        pub mod himushi;
        pub mod horncrier;
        pub mod inbyo;
        pub mod kuongatari;
        pub mod man_toyer;
        pub mod octoliar;
        pub mod orb_piercer;
        pub mod ottobas;
        pub mod shiratsubi;
        pub mod silkfang;
        pub mod spittleshrub;
        pub mod stingerhead;
        pub mod swarmhoarder;
        pub mod swarmshocker;
        pub mod tachikatana;
        pub mod uibuta;
        pub mod vinebinder;
        pub mod vineblaster;
        pub mod vinepoofer;
        pub mod virinsipede;
        pub mod woodcrier;
        pub mod yomotsubi;
    }

    /// Passive mob kinds.
    pub mod passive {
        pub mod abyssal_snail;
        pub mod ashimite;
        pub mod billabonk;
        pub mod fuwagi;
        pub mod ikonia;
        pub mod kakatsumuri;
        pub mod neritantan;
        pub mod okibo;
        pub mod sakuranbo;
        pub mod shroombear;
        pub mod tesuchi;
    }
}

/// The plugin's own items, built on top of the vanilla ones.
///
/// Like a custom mob, a custom item is a vanilla item wearing an identity: a base item, a
/// visible name, whatever `apply` adds, and a marker in the stack's own persistent data.
/// `item_builder` is the trait an item implements and where that marker is written and read
/// back; `abyss_relic` is the worked example, reachable in game through `/abyss item relic`.
mod items {
    pub mod abyss_relic;
    pub mod item_builder;
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
pub const LAYERS: [Layer; 8] = Layer::ALL;

/// Every mob kind the plugin defines, in id order.
///
/// A kind's associated functions cannot be called through a trait object, so the catalogue
/// spells each one out with `MobKind::of`, which fills the entry from that kind's own
/// `MobBuilder` methods. Adding a kind therefore means adding it here, and `/abyss summon`
/// can then reach it by name.
///
/// A function rather than a `const`: `MobKind::of` calls trait methods, which are not
/// `const`, so the catalogue is built once at first use.
#[must_use]
pub fn mob_kinds() -> [mobs::mob_builder::MobKind; 65] {
    [
        mobs::mob_builder::MobKind::of::<mobs::docile::beniguma::Beniguma>("Beniguma"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::cyatoria::Cyatoria>("Cyatoria"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::small_hammerbeak::SmallHammerbeak>(
            "Small Hammerbeak",
        ),
        mobs::mob_builder::MobKind::of::<mobs::aerial::giant_hammerbeak::GiantHammerbeak>(
            "Giant Hammerbeak",
        ),
        mobs::mob_builder::MobKind::of::<mobs::aerial::splitjaw::Splitjaw>("Splitjaw"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::woodcrier::Woodcrier>("Woodcrier"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::man_toyer::ManToyer>("Man-toyer"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::silkfang::Silkfang>("Silkfang"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::spittleshrub::Spittleshrub>("Spittleshrub"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::vinebinder::Vinebinder>("Vinebinder"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::vineblaster::Vineblaster>("Vineblaster"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::vinepoofer::Vinepoofer>("Vinepoofer"),
        mobs::mob_builder::MobKind::of::<mobs::passive::fuwagi::Fuwagi>("Fuwagi"),
        mobs::mob_builder::MobKind::of::<mobs::passive::okibo::Okibo>("Okibo"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::spiralsquid::Spiralsquid>("Spiralsquid"),
        mobs::mob_builder::MobKind::of::<mobs::docile::chimokami::Chimokami>("Chimokami"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::corpse_weeper::CorpseWeeper>(
            "Corpse-Weeper",
        ),
        mobs::mob_builder::MobKind::of::<mobs::aerial::dosetori::Dosetori>("Dosetori"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::ottobas::Ottobas>("Ottobas"),
        mobs::mob_builder::MobKind::of::<mobs::passive::tesuchi::Tesuchi>("Tesuchi"),
        mobs::mob_builder::MobKind::of::<mobs::docile::hisoishi::Hisoishi>("Hisoishi"),
        mobs::mob_builder::MobKind::of::<mobs::docile::rockwalker::Rockwalker>("Rockwalker"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::rohana::Rohana>("Rohana"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::stinger::Stinger>("Stinger"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::inbyo::Inbyo>("Inbyo"),
        mobs::mob_builder::MobKind::of::<mobs::passive::billabonk::Billabonk>("Billabonk"),
        mobs::mob_builder::MobKind::of::<mobs::passive::neritantan::Neritantan>("Neritantan"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::ikenosuke::Ikenosuke>("Ikenosuke"),
        mobs::mob_builder::MobKind::of::<mobs::docile::daikonchu::Daikonchu>("Daikonchu"),
        mobs::mob_builder::MobKind::of::<mobs::docile::rinsipede::Rinsipede>("Rinsipede"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::buroguro::Buroguro>("Buroguro"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::madokajack::Madokajack>("Madokajack"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::squidfly::Squidfly>("Squidfly"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::himushi::Himushi>("Himushi"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::yomotsubi::Yomotsubi>("Yomotsubi"),
        mobs::mob_builder::MobKind::of::<mobs::passive::kakatsumuri::Kakatsumuri>("Kakatsumuri"),
        mobs::mob_builder::MobKind::of::<mobs::passive::sakuranbo::Sakuranbo>("Sakuranbo"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::hashofu::Hashofu>("Hashofu"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::onitsuchi::Onitsuchi>("Onitsuchi"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::horncrier::Horncrier>("Horncrier"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::kuongatari::Kuongatari>("Kuongatari"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::orb_piercer::OrbPiercer>("Orb Piercer"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::swarmshocker::Swarmshocker>("Swarmshocker"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::tachikatana::Tachikatana>("Tachikatana"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::uibuta::Uibuta>("Uibuta"),
        mobs::mob_builder::MobKind::of::<mobs::passive::shroombear::Shroombear>("Shroombear"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::hagamizukin::Hagamizukin>("Hagamizukin"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::kazura::Kazura>("Kazura"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::makihige::Makihige>("Makihige"),
        mobs::mob_builder::MobKind::of::<mobs::docile::spikewalker::Spikewalker>("Spikewalker"),
        mobs::mob_builder::MobKind::of::<mobs::docile::titanjaw::Titanjaw>("Titanjaw"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::kagi_tachikiri::KagiTachikiri>(
            "Kagi-Tachikiri",
        ),
        mobs::mob_builder::MobKind::of::<mobs::hostile::chikyuoebi::Chikyuoebi>("Chikyuoebi"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::octoliar::Octoliar>("Octoliar"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::stingerhead::Stingerhead>("Stingerhead"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::swarmhoarder::Swarmhoarder>("Swarmhoarder"),
        mobs::mob_builder::MobKind::of::<mobs::aerial::spore_manta::SporeManta>("Spore Manta"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::virinsipede::Virinsipede>("Virinsipede"),
        mobs::mob_builder::MobKind::of::<mobs::hostile::shiratsubi::Shiratsubi>("Shiratsubi"),
        mobs::mob_builder::MobKind::of::<mobs::passive::abyssal_snail::AbyssalSnail>(
            "Abyssal Snail",
        ),
        mobs::mob_builder::MobKind::of::<mobs::passive::ashimite::Ashimite>("Ashimite"),
        mobs::mob_builder::MobKind::of::<mobs::passive::ikonia::Ikonia>("Ikonia"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::demonfish::Demonfish>("Demonfish"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::edusan::Edusan>("Edusan"),
        mobs::mob_builder::MobKind::of::<mobs::aquatic::hisomukuro::Hisomukuro>("Hisomukuro"),
    ]
}

/// Returns the mob kind called `name`, matched case-insensitively against the display name
/// and with spaces or hyphens interchangeable.
#[must_use]
pub(crate) fn mob_kind(name: &str) -> Option<mobs::mob_builder::MobKind> {
    let wanted = normalise(name);
    mob_kinds()
        .into_iter()
        .find(|kind| normalise(kind.name) == wanted)
}

/// Folds a mob name to a form that ignores case, spaces and hyphens.
fn normalise(name: &str) -> String {
    name.chars()
        .filter(|c| *c != ' ' && *c != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

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

    fn mobs() {
        let started = Instant::now();

        let mobs = mob_kinds();

        info!(
            "Registered {} custom mob(s) in {:?}",
            mobs.len(),
            started.elapsed()
        );
    }

    fn events(context: &Context) {
        let events: [fn(&Context); 6] = [
            events::layer_join::LayerJoin::register,
            events::layer_leave::LayerLeave::register,
            events::fall_damage::FallDamage::register,
            events::layer_respawn::LayerRespawn::register,
            events::layer_teleport::LayerTeleport::register,
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
        Self::mobs();
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
