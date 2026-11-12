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
/// [`EntityBuilder`] a kind spawns itself with, where that marker is written and read back.
///
/// What an entity *is* and where it may appear are then each kind's own business: a mob
/// declares its `LAYERS` and `CATEGORY` beside its own `spawn`, and `mobs::mob_kind` is the
/// plain entry [`mob_kinds`] collects for the creature-spawn rule. A trader has neither —
/// `villagers::boy` is a plain constructor over a spawned entity. Mobs are filed one folder
/// per [`MobCategory`].
mod entities {
    pub(crate) mod entity_builder;

    /// Mobs, the creatures of the Abyss.
    pub(crate) mod mobs {
        pub(crate) mod mob_category;
        pub(crate) mod mob_kind;

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
    ///
    /// Kept whole while `/abyss summon` is gone: the kinds are authored and their numbers
    /// reserved, but nothing places one until a trader has a caller again — a map marker, or
    /// a command of its own.
    #[expect(
        dead_code,
        reason = "no caller places a trader until the summon subcommand returns"
    )]
    pub(crate) mod villagers {
        pub(crate) mod boy;
    }
}

/// The plugin's own items, built on top of the vanilla ones.
///
/// Like a custom mob, a custom item is a vanilla item wearing an identity: a base item, a
/// visible name, whatever the kind's constructor sets, and a marker in the stack's own
/// persistent data. `item_builder` is the chainable builder and where that marker is written
/// and read back; `orthcoin` is the worked example, an inherent `stack()` rather than a trait
/// impl — see `item_builder` for why there is no trait.
///
/// Nothing creates an item yet. `/abyss item`, which gave one to the sender so the path could
/// be exercised end to end, has been removed, so the tree is kept whole but unused until a
/// real source of items — loot tables, crafting, a shop — exists.
#[expect(
    dead_code,
    reason = "no caller creates an item until a real item source exists"
)]
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
/// Each entry names a kind's own `LAYERS`, `CATEGORY` and `spawn`, so the values stay
/// written once in the kind's file and this list only has to name them. Adding a kind means
/// adding it here as well as to its folder's module list.
///
/// Villagers are **not** listed: they share the identity marker but not the question a mob
/// catalogue answers — where a layer's wild life comes from — so a trader is placed where
/// its shop stands rather than drawn by the creature-spawn rule.
///
/// It is long by nature — one entry per kind — so the line limit is waived here rather than
/// by splitting the list into several functions that would each still need this attribute.
#[expect(
    clippy::too_many_lines,
    reason = "a catalogue is one entry per kind by design"
)]
#[must_use]
pub(crate) fn mob_kinds() -> [entities::mobs::mob_kind::MobKind; 65] {
    [
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::beniguma::Beniguma::LAYERS,
            entities::mobs::docile::beniguma::Beniguma::CATEGORY,
            entities::mobs::docile::beniguma::Beniguma::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::cyatoria::Cyatoria::LAYERS,
            entities::mobs::aerial::cyatoria::Cyatoria::CATEGORY,
            entities::mobs::aerial::cyatoria::Cyatoria::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::small_hammerbeak::SmallHammerbeak::LAYERS,
            entities::mobs::aerial::small_hammerbeak::SmallHammerbeak::CATEGORY,
            entities::mobs::aerial::small_hammerbeak::SmallHammerbeak::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::giant_hammerbeak::GiantHammerbeak::LAYERS,
            entities::mobs::aerial::giant_hammerbeak::GiantHammerbeak::CATEGORY,
            entities::mobs::aerial::giant_hammerbeak::GiantHammerbeak::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::splitjaw::Splitjaw::LAYERS,
            entities::mobs::aerial::splitjaw::Splitjaw::CATEGORY,
            entities::mobs::aerial::splitjaw::Splitjaw::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::woodcrier::Woodcrier::LAYERS,
            entities::mobs::hostile::woodcrier::Woodcrier::CATEGORY,
            entities::mobs::hostile::woodcrier::Woodcrier::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::man_toyer::ManToyer::LAYERS,
            entities::mobs::hostile::man_toyer::ManToyer::CATEGORY,
            entities::mobs::hostile::man_toyer::ManToyer::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::silkfang::Silkfang::LAYERS,
            entities::mobs::hostile::silkfang::Silkfang::CATEGORY,
            entities::mobs::hostile::silkfang::Silkfang::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::spittleshrub::Spittleshrub::LAYERS,
            entities::mobs::hostile::spittleshrub::Spittleshrub::CATEGORY,
            entities::mobs::hostile::spittleshrub::Spittleshrub::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::vinebinder::Vinebinder::LAYERS,
            entities::mobs::hostile::vinebinder::Vinebinder::CATEGORY,
            entities::mobs::hostile::vinebinder::Vinebinder::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::vineblaster::Vineblaster::LAYERS,
            entities::mobs::hostile::vineblaster::Vineblaster::CATEGORY,
            entities::mobs::hostile::vineblaster::Vineblaster::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::vinepoofer::Vinepoofer::LAYERS,
            entities::mobs::hostile::vinepoofer::Vinepoofer::CATEGORY,
            entities::mobs::hostile::vinepoofer::Vinepoofer::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::fuwagi::Fuwagi::LAYERS,
            entities::mobs::passive::fuwagi::Fuwagi::CATEGORY,
            entities::mobs::passive::fuwagi::Fuwagi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::okibo::Okibo::LAYERS,
            entities::mobs::passive::okibo::Okibo::CATEGORY,
            entities::mobs::passive::okibo::Okibo::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::spiralsquid::Spiralsquid::LAYERS,
            entities::mobs::aquatic::spiralsquid::Spiralsquid::CATEGORY,
            entities::mobs::aquatic::spiralsquid::Spiralsquid::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::chimokami::Chimokami::LAYERS,
            entities::mobs::docile::chimokami::Chimokami::CATEGORY,
            entities::mobs::docile::chimokami::Chimokami::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::corpse_weeper::CorpseWeeper::LAYERS,
            entities::mobs::aerial::corpse_weeper::CorpseWeeper::CATEGORY,
            entities::mobs::aerial::corpse_weeper::CorpseWeeper::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::dosetori::Dosetori::LAYERS,
            entities::mobs::aerial::dosetori::Dosetori::CATEGORY,
            entities::mobs::aerial::dosetori::Dosetori::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::ottobas::Ottobas::LAYERS,
            entities::mobs::hostile::ottobas::Ottobas::CATEGORY,
            entities::mobs::hostile::ottobas::Ottobas::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::tesuchi::Tesuchi::LAYERS,
            entities::mobs::passive::tesuchi::Tesuchi::CATEGORY,
            entities::mobs::passive::tesuchi::Tesuchi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::hisoishi::Hisoishi::LAYERS,
            entities::mobs::docile::hisoishi::Hisoishi::CATEGORY,
            entities::mobs::docile::hisoishi::Hisoishi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::rockwalker::Rockwalker::LAYERS,
            entities::mobs::docile::rockwalker::Rockwalker::CATEGORY,
            entities::mobs::docile::rockwalker::Rockwalker::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::rohana::Rohana::LAYERS,
            entities::mobs::aerial::rohana::Rohana::CATEGORY,
            entities::mobs::aerial::rohana::Rohana::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::stinger::Stinger::LAYERS,
            entities::mobs::aerial::stinger::Stinger::CATEGORY,
            entities::mobs::aerial::stinger::Stinger::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::inbyo::Inbyo::LAYERS,
            entities::mobs::hostile::inbyo::Inbyo::CATEGORY,
            entities::mobs::hostile::inbyo::Inbyo::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::billabonk::Billabonk::LAYERS,
            entities::mobs::passive::billabonk::Billabonk::CATEGORY,
            entities::mobs::passive::billabonk::Billabonk::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::neritantan::Neritantan::LAYERS,
            entities::mobs::passive::neritantan::Neritantan::CATEGORY,
            entities::mobs::passive::neritantan::Neritantan::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::ikenosuke::Ikenosuke::LAYERS,
            entities::mobs::aquatic::ikenosuke::Ikenosuke::CATEGORY,
            entities::mobs::aquatic::ikenosuke::Ikenosuke::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::daikonchu::Daikonchu::LAYERS,
            entities::mobs::docile::daikonchu::Daikonchu::CATEGORY,
            entities::mobs::docile::daikonchu::Daikonchu::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::rinsipede::Rinsipede::LAYERS,
            entities::mobs::docile::rinsipede::Rinsipede::CATEGORY,
            entities::mobs::docile::rinsipede::Rinsipede::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::buroguro::Buroguro::LAYERS,
            entities::mobs::aerial::buroguro::Buroguro::CATEGORY,
            entities::mobs::aerial::buroguro::Buroguro::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::madokajack::Madokajack::LAYERS,
            entities::mobs::aerial::madokajack::Madokajack::CATEGORY,
            entities::mobs::aerial::madokajack::Madokajack::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::squidfly::Squidfly::LAYERS,
            entities::mobs::aerial::squidfly::Squidfly::CATEGORY,
            entities::mobs::aerial::squidfly::Squidfly::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::himushi::Himushi::LAYERS,
            entities::mobs::hostile::himushi::Himushi::CATEGORY,
            entities::mobs::hostile::himushi::Himushi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::yomotsubi::Yomotsubi::LAYERS,
            entities::mobs::hostile::yomotsubi::Yomotsubi::CATEGORY,
            entities::mobs::hostile::yomotsubi::Yomotsubi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::kakatsumuri::Kakatsumuri::LAYERS,
            entities::mobs::passive::kakatsumuri::Kakatsumuri::CATEGORY,
            entities::mobs::passive::kakatsumuri::Kakatsumuri::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::sakuranbo::Sakuranbo::LAYERS,
            entities::mobs::passive::sakuranbo::Sakuranbo::CATEGORY,
            entities::mobs::passive::sakuranbo::Sakuranbo::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::hashofu::Hashofu::LAYERS,
            entities::mobs::aerial::hashofu::Hashofu::CATEGORY,
            entities::mobs::aerial::hashofu::Hashofu::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::onitsuchi::Onitsuchi::LAYERS,
            entities::mobs::aerial::onitsuchi::Onitsuchi::CATEGORY,
            entities::mobs::aerial::onitsuchi::Onitsuchi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::horncrier::Horncrier::LAYERS,
            entities::mobs::hostile::horncrier::Horncrier::CATEGORY,
            entities::mobs::hostile::horncrier::Horncrier::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::kuongatari::Kuongatari::LAYERS,
            entities::mobs::hostile::kuongatari::Kuongatari::CATEGORY,
            entities::mobs::hostile::kuongatari::Kuongatari::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::orb_piercer::OrbPiercer::LAYERS,
            entities::mobs::hostile::orb_piercer::OrbPiercer::CATEGORY,
            entities::mobs::hostile::orb_piercer::OrbPiercer::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::swarmshocker::Swarmshocker::LAYERS,
            entities::mobs::hostile::swarmshocker::Swarmshocker::CATEGORY,
            entities::mobs::hostile::swarmshocker::Swarmshocker::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::tachikatana::Tachikatana::LAYERS,
            entities::mobs::hostile::tachikatana::Tachikatana::CATEGORY,
            entities::mobs::hostile::tachikatana::Tachikatana::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::uibuta::Uibuta::LAYERS,
            entities::mobs::hostile::uibuta::Uibuta::CATEGORY,
            entities::mobs::hostile::uibuta::Uibuta::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::shroombear::Shroombear::LAYERS,
            entities::mobs::passive::shroombear::Shroombear::CATEGORY,
            entities::mobs::passive::shroombear::Shroombear::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::hagamizukin::Hagamizukin::LAYERS,
            entities::mobs::aquatic::hagamizukin::Hagamizukin::CATEGORY,
            entities::mobs::aquatic::hagamizukin::Hagamizukin::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::kazura::Kazura::LAYERS,
            entities::mobs::aquatic::kazura::Kazura::CATEGORY,
            entities::mobs::aquatic::kazura::Kazura::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::makihige::Makihige::LAYERS,
            entities::mobs::aquatic::makihige::Makihige::CATEGORY,
            entities::mobs::aquatic::makihige::Makihige::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::spikewalker::Spikewalker::LAYERS,
            entities::mobs::docile::spikewalker::Spikewalker::CATEGORY,
            entities::mobs::docile::spikewalker::Spikewalker::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::docile::titanjaw::Titanjaw::LAYERS,
            entities::mobs::docile::titanjaw::Titanjaw::CATEGORY,
            entities::mobs::docile::titanjaw::Titanjaw::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::kagi_tachikiri::KagiTachikiri::LAYERS,
            entities::mobs::aerial::kagi_tachikiri::KagiTachikiri::CATEGORY,
            entities::mobs::aerial::kagi_tachikiri::KagiTachikiri::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::chikyuoebi::Chikyuoebi::LAYERS,
            entities::mobs::hostile::chikyuoebi::Chikyuoebi::CATEGORY,
            entities::mobs::hostile::chikyuoebi::Chikyuoebi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::octoliar::Octoliar::LAYERS,
            entities::mobs::hostile::octoliar::Octoliar::CATEGORY,
            entities::mobs::hostile::octoliar::Octoliar::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::stingerhead::Stingerhead::LAYERS,
            entities::mobs::hostile::stingerhead::Stingerhead::CATEGORY,
            entities::mobs::hostile::stingerhead::Stingerhead::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::swarmhoarder::Swarmhoarder::LAYERS,
            entities::mobs::hostile::swarmhoarder::Swarmhoarder::CATEGORY,
            entities::mobs::hostile::swarmhoarder::Swarmhoarder::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aerial::spore_manta::SporeManta::LAYERS,
            entities::mobs::aerial::spore_manta::SporeManta::CATEGORY,
            entities::mobs::aerial::spore_manta::SporeManta::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::virinsipede::Virinsipede::LAYERS,
            entities::mobs::hostile::virinsipede::Virinsipede::CATEGORY,
            entities::mobs::hostile::virinsipede::Virinsipede::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::hostile::shiratsubi::Shiratsubi::LAYERS,
            entities::mobs::hostile::shiratsubi::Shiratsubi::CATEGORY,
            entities::mobs::hostile::shiratsubi::Shiratsubi::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::abyssal_snail::AbyssalSnail::LAYERS,
            entities::mobs::passive::abyssal_snail::AbyssalSnail::CATEGORY,
            entities::mobs::passive::abyssal_snail::AbyssalSnail::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::ashimite::Ashimite::LAYERS,
            entities::mobs::passive::ashimite::Ashimite::CATEGORY,
            entities::mobs::passive::ashimite::Ashimite::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::passive::ikonia::Ikonia::LAYERS,
            entities::mobs::passive::ikonia::Ikonia::CATEGORY,
            entities::mobs::passive::ikonia::Ikonia::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::demonfish::Demonfish::LAYERS,
            entities::mobs::aquatic::demonfish::Demonfish::CATEGORY,
            entities::mobs::aquatic::demonfish::Demonfish::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::edusan::Edusan::LAYERS,
            entities::mobs::aquatic::edusan::Edusan::CATEGORY,
            entities::mobs::aquatic::edusan::Edusan::spawn,
        ),
        entities::mobs::mob_kind::MobKind::of(
            entities::mobs::aquatic::hisomukuro::Hisomukuro::LAYERS,
            entities::mobs::aquatic::hisomukuro::Hisomukuro::CATEGORY,
            entities::mobs::aquatic::hisomukuro::Hisomukuro::spawn,
        ),
    ]
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
