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
    pub(crate) mod abyss;
    pub(crate) mod command;
}

/// The plugin's own entities, built on top of the vanilla ones.
///
/// Minecraft's entity types cannot be extended, so an entity here is a vanilla entity
/// wearing an identity: a base type, a visible name, scaled attributes, and a marker in its
/// own persistent data. `entity_builder` is the core shared by every kind: it holds the
/// [`EntityBuilder`] trait, where that marker is written and read back.
///
/// What an entity *is* and where it may appear are then each builder's own business.
/// `mobs::mob_builder` adds a mob's [`MobBuilder`] with its [`Category`] and its layers, and
/// `villagers::villager_builder` adds a villager's [`VillagerBuilder`] with its profession.
/// Mobs are filed one folder per [`Category`], and `mobs::docile::beniguma` is the worked
/// example; `villagers::boy` is the villager one.
mod entities {
    pub(crate) mod entity_builder;

    /// Mobs, the creatures of the Abyss.
    pub(crate) mod mobs {
        pub(crate) mod mob_builder;

        /// Aerial mob kinds.
        pub(crate) mod aerial {
            pub(crate) mod buroguro;
            pub(crate) mod corpse_weeper;
            pub(crate) mod cyatoria;
            pub(crate) mod dosetori;
            pub(crate) mod giant_hammerbeak;
            pub(crate) mod hashofu;
            pub(crate) mod kagi_tachikiri;
            pub(crate) mod madokajack;
            pub(crate) mod onitsuchi;
            pub(crate) mod rohana;
            pub(crate) mod small_hammerbeak;
            pub(crate) mod splitjaw;
            pub(crate) mod spore_manta;
            pub(crate) mod squidfly;
            pub(crate) mod stinger;
        }

        /// Aquatic mob kinds.
        pub(crate) mod aquatic {
            pub(crate) mod demonfish;
            pub(crate) mod edusan;
            pub(crate) mod hagamizukin;
            pub(crate) mod hisomukuro;
            pub(crate) mod ikenosuke;
            pub(crate) mod kazura;
            pub(crate) mod makihige;
            pub(crate) mod spiralsquid;
        }

        /// Docile mob kinds.
        pub(crate) mod docile {
            pub(crate) mod beniguma;
            pub(crate) mod chimokami;
            pub(crate) mod daikonchu;
            pub(crate) mod hisoishi;
            pub(crate) mod rinsipede;
            pub(crate) mod rockwalker;
            pub(crate) mod spikewalker;
            pub(crate) mod titanjaw;
        }

        /// Hostile mob kinds.
        pub(crate) mod hostile {
            pub(crate) mod chikyuoebi;
            pub(crate) mod himushi;
            pub(crate) mod horncrier;
            pub(crate) mod inbyo;
            pub(crate) mod kuongatari;
            pub(crate) mod man_toyer;
            pub(crate) mod octoliar;
            pub(crate) mod orb_piercer;
            pub(crate) mod ottobas;
            pub(crate) mod shiratsubi;
            pub(crate) mod silkfang;
            pub(crate) mod spittleshrub;
            pub(crate) mod stingerhead;
            pub(crate) mod swarmhoarder;
            pub(crate) mod swarmshocker;
            pub(crate) mod tachikatana;
            pub(crate) mod uibuta;
            pub(crate) mod vinebinder;
            pub(crate) mod vineblaster;
            pub(crate) mod vinepoofer;
            pub(crate) mod virinsipede;
            pub(crate) mod woodcrier;
            pub(crate) mod yomotsubi;
        }

        /// Passive mob kinds.
        pub(crate) mod passive {
            pub(crate) mod abyssal_snail;
            pub(crate) mod ashimite;
            pub(crate) mod billabonk;
            pub(crate) mod fuwagi;
            pub(crate) mod ikonia;
            pub(crate) mod kakatsumuri;
            pub(crate) mod neritantan;
            pub(crate) mod okibo;
            pub(crate) mod sakuranbo;
            pub(crate) mod shroombear;
            pub(crate) mod tesuchi;
        }
    }

    /// Villagers, the traders of Orth.
    pub(crate) mod villagers {
        pub(crate) mod boy;
        pub(crate) mod villager_builder;
    }
}

/// The plugin's own items, built on top of the vanilla ones.
///
/// Like a custom mob, a custom item is a vanilla item wearing an identity: a base item, a
/// visible name, whatever the kind's constructor sets, and a marker in the stack's own
/// persistent data. `item_builder` is the chainable builder and where that marker is written
/// and read back; `orthcoin` is the worked example, an inherent `stack()` rather than a trait
/// impl — see `item_builder` for why there is no trait.
mod items {
    pub(crate) mod item_builder;
    pub(crate) mod orthcoin;
}

/// The layer worlds, and the machinery that keeps them usable.
///
/// A layer world is only ever as good as the three pieces around it: it has to be created
/// ([`setup`]), it has to generate empty until its authored map exists ([`generator`]),
/// and it has to share the surface's clock with the rest of the Abyss ([`clock`]).
mod world {
    pub(crate) mod clock;
    pub(crate) mod generator;
    pub(crate) mod setup;
}

/// Small helpers shared across the plugin, split by what each module owns.
///
/// The three `thread_local` records a transition needs are each a module of their own, so
/// the state and the functions that maintain it cannot drift apart. Every submodule is
/// re-exported here, so a caller writes `use crate::utils::{carry_fall, layer_of};` and
/// does not have to know which file a helper ended up in; reach for `utils::fall::…`
/// directly only when the grouping is what you are looking for.
mod utils {
    pub(crate) mod fall;
    pub(crate) mod general;
    pub(crate) mod teleport;
    pub(crate) mod text;
    pub(crate) mod transition;
    pub(crate) mod world;

    pub(crate) use fall::{carry_fall, clear_fall, resume_fall};
    pub(crate) use teleport::{arrived_by_teleport, note_teleport};
    pub(crate) use text::{ordinal, rgb, tell};
    pub(crate) use transition::{begin_transition, end_transition, ignores_transitions};
    pub(crate) use world::{layer_of, world_for};
}

/// Event handlers for `mine-in-abyss`.
mod events {
    pub(crate) mod bed_spawn;
    pub(crate) mod creature_spawn;
    pub(crate) mod curse;
    pub(crate) mod event;
    pub(crate) mod fall_damage;
    pub(crate) mod layer_join;
    pub(crate) mod layer_leave;
    pub(crate) mod layer_respawn;
    pub(crate) mod layer_teleport;
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
/// Villagers are **not** listed here. They share the identity marker but not the question a
/// mob catalogue answers — where a layer's wild life comes from — so a trader is placed on
/// the map rather than drawn by the creature-spawn rule, and the villager kinds live under
/// `entities::villagers`, each claiming its number from `villager_builder::ids`.
///
/// A function rather than a `const`: `MobKind::of` calls trait methods, which are not
/// `const`, so the catalogue is built once at first use.
///
/// It is long by nature — one line per kind, and 66 of them — so the line limit is waived
/// here rather than by splitting the list into several functions that would each still
/// need this attribute.
#[expect(
    clippy::too_many_lines,
    reason = "a catalogue is one entry per kind by design"
)]
#[must_use]
pub fn mob_kinds() -> [entities::mobs::mob_builder::MobKind; 65] {
    [
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::beniguma::Beniguma>(
            "Beniguma",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::cyatoria::Cyatoria>(
            "Cyatoria",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::aerial::small_hammerbeak::SmallHammerbeak,
        >("Small Hammerbeak"),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::aerial::giant_hammerbeak::GiantHammerbeak,
        >("Giant Hammerbeak"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::splitjaw::Splitjaw>(
            "Splitjaw",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::woodcrier::Woodcrier>(
            "Woodcrier",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::man_toyer::ManToyer>(
            "Man-toyer",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::silkfang::Silkfang>(
            "Silkfang",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::hostile::spittleshrub::Spittleshrub,
        >("Spittleshrub"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::vinebinder::Vinebinder>(
            "Vinebinder",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::vineblaster::Vineblaster>(
            "Vineblaster",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::vinepoofer::Vinepoofer>(
            "Vinepoofer",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::fuwagi::Fuwagi>(
            "Fuwagi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::okibo::Okibo>("Okibo"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::spiralsquid::Spiralsquid>(
            "Spiralsquid",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::chimokami::Chimokami>(
            "Chimokami",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::aerial::corpse_weeper::CorpseWeeper,
        >("Corpse-Weeper"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::dosetori::Dosetori>(
            "Dosetori",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::ottobas::Ottobas>(
            "Ottobas",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::tesuchi::Tesuchi>(
            "Tesuchi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::hisoishi::Hisoishi>(
            "Hisoishi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::rockwalker::Rockwalker>(
            "Rockwalker",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::rohana::Rohana>(
            "Rohana",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::stinger::Stinger>(
            "Stinger",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::inbyo::Inbyo>("Inbyo"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::billabonk::Billabonk>(
            "Billabonk",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::neritantan::Neritantan>(
            "Neritantan",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::ikenosuke::Ikenosuke>(
            "Ikenosuke",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::daikonchu::Daikonchu>(
            "Daikonchu",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::rinsipede::Rinsipede>(
            "Rinsipede",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::buroguro::Buroguro>(
            "Buroguro",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::madokajack::Madokajack>(
            "Madokajack",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::squidfly::Squidfly>(
            "Squidfly",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::himushi::Himushi>(
            "Himushi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::yomotsubi::Yomotsubi>(
            "Yomotsubi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::kakatsumuri::Kakatsumuri>(
            "Kakatsumuri",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::sakuranbo::Sakuranbo>(
            "Sakuranbo",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::hashofu::Hashofu>(
            "Hashofu",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::onitsuchi::Onitsuchi>(
            "Onitsuchi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::horncrier::Horncrier>(
            "Horncrier",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::kuongatari::Kuongatari>(
            "Kuongatari",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::orb_piercer::OrbPiercer>(
            "Orb Piercer",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::hostile::swarmshocker::Swarmshocker,
        >("Swarmshocker"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::tachikatana::Tachikatana>(
            "Tachikatana",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::uibuta::Uibuta>(
            "Uibuta",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::shroombear::Shroombear>(
            "Shroombear",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::hagamizukin::Hagamizukin>(
            "Hagamizukin",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::kazura::Kazura>(
            "Kazura",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::makihige::Makihige>(
            "Makihige",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::spikewalker::Spikewalker>(
            "Spikewalker",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::titanjaw::Titanjaw>(
            "Titanjaw",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::aerial::kagi_tachikiri::KagiTachikiri,
        >("Kagi-Tachikiri"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::chikyuoebi::Chikyuoebi>(
            "Chikyuoebi",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::octoliar::Octoliar>(
            "Octoliar",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::stingerhead::Stingerhead>(
            "Stingerhead",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::hostile::swarmhoarder::Swarmhoarder,
        >("Swarmhoarder"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aerial::spore_manta::SporeManta>(
            "Spore Manta",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::virinsipede::Virinsipede>(
            "Virinsipede",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::hostile::shiratsubi::Shiratsubi>(
            "Shiratsubi",
        ),
        entities::mobs::mob_builder::MobKind::of::<
            entities::mobs::passive::abyssal_snail::AbyssalSnail,
        >("Abyssal Snail"),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::ashimite::Ashimite>(
            "Ashimite",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::passive::ikonia::Ikonia>(
            "Ikonia",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::demonfish::Demonfish>(
            "Demonfish",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::edusan::Edusan>(
            "Edusan",
        ),
        entities::mobs::mob_builder::MobKind::of::<entities::mobs::aquatic::hisomukuro::Hisomukuro>(
            "Hisomukuro",
        ),
    ]
}

/// Returns the mob kind called `name`, matched case-insensitively against the display name
/// and with spaces or hyphens interchangeable.
#[must_use]
pub(crate) fn mob_kind(name: &str) -> Option<entities::mobs::mob_builder::MobKind> {
    let wanted = normalise(name);
    mob_kinds()
        .into_iter()
        .find(|kind| normalise(kind.name) == wanted)
}

/// Every villager kind the plugin defines.
///
/// The trading counterpart of [`mob_kinds`](crate::mob_kinds), and the same shape: a
/// `VillagerKind` per trader, filled from the kind's own trait methods. A villager's number
/// is claimed from the shared marker space, so the ids here must not repeat any in
/// `mob_kinds` — mobs hold `1`–`65`, and traders start above them.
#[must_use]
pub(crate) fn villager_kinds() -> [entities::villagers::villager_builder::VillagerKind; 1] {
    [entities::villagers::villager_builder::VillagerKind::of::<
        entities::villagers::boy::Boy,
    >("Boy, Lord of Currency")]
}

/// Returns the villager kind called `name`, matched the same way [`mob_kind`] matches.
#[must_use]
pub(crate) fn villager_kind(
    name: &str,
) -> Option<entities::villagers::villager_builder::VillagerKind> {
    let wanted = normalise(name);
    villager_kinds()
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
        let events: [fn(&Context); 7] = [
            events::layer_join::LayerJoin::register,
            events::layer_leave::LayerLeave::register,
            events::fall_damage::FallDamage::register,
            events::layer_respawn::LayerRespawn::register,
            events::layer_teleport::LayerTeleport::register,
            events::bed_spawn::BedSpawn::register,
            events::creature_spawn::CreatureSpawn::register,
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
                .map(ToString::to_string)
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

/// The plugin entry point the host calls by export name.
///
/// The body is a macro expansion, so the generated `__init_plugin` cannot carry its
/// own rustdoc. The expectation is scoped to this module rather than the invocation,
/// because an attribute on a macro call does not reach what it expands to.
///
/// The module is `pub` because the expansion defines `pub extern "C" fn
/// __init_plugin`, and the host calls it by export name.
#[expect(
    missing_docs,
    reason = "the generated __init_plugin is documented by the macro, not here"
)]
pub mod entry {
    use crate::MineInAbyss;

    pumpkin_plugin_api::register_plugin!(MineInAbyss);
}
