//! Replaces the vanilla mobs that spawn in a layer with the Abyss's own.
//!
//! Every layer world is a hand-built map of the Abyss, so a plain zombie or cow wandering
//! into it would be an artefact of the generator rather than a part of the setting. A natural
//! spawn in a layer world is therefore **cancelled**, and a custom kind is spawned in its
//! place.
//!
//! Which kind is decided by [`MobBuilder::layers`](crate::entities::mobs::mob_builder::MobBuilder::layers):
//! only a kind that lists the layer may appear there, so the same list that used to guard
//! `/abyss summon` decides what lives where. A layer with no eligible kind stays empty,
//! which is the honest answer — nothing was authored to live there yet.
//!
//! The replacement is placed at the event's own position and world, so it stands exactly where
//! the vanilla mob would have: this handler changes *what* spawns, never *where*.
//!
//! Only the spawn reasons that mean "the world decided to spawn this" are considered. A
//! spawner, a breeding pair, a spawn egg or a command is a deliberate act and is left alone,
//! so this cannot interfere with a player or an operator placing a mob on purpose.
//!
//! Entities do not descend between layers, so a mob spawned here stays in the layer it was
//! born in for its whole life — see the notes in `AGENTS.md`.

use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::entity::creature_spawn::CreatureSpawnEvent;
use pumpkin_plugin_api::events_wit::CreatureSpawnEventData;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::{Context, Server};

use crate::entities::mobs::mob_builder::MobKind;
use crate::events::event::Event;

/// Replaces a naturally spawned vanilla mob in a layer world with a custom kind.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct CreatureSpawn;

impl Event for CreatureSpawn {
    fn register(context: &Context) {
        context
            .register_event_handler::<CreatureSpawnEvent, _>(
                CreatureSpawn,
                Self::PRIORITY,
                Self::BLOCKING,
            )
            .unwrap_or_else(|_| Self::fail("CreatureSpawn"));
    }
}

impl EventHandler<CreatureSpawnEvent> for CreatureSpawn {
    fn handle(&self, _server: Server, event: CreatureSpawnEventData) -> CreatureSpawnEventData {
        if !is_natural(&event.spawn_reason) {
            note_reason(&event.spawn_reason);
            return event;
        }

        let Some(layer) = crate::layer_for_world(&event.target_world.get_name()) else {
            return event;
        };

        if let Some(kind) = choose(layer, &event.position) {
            let (x, y, z) = event.position;
            (kind.spawn)(&event.target_world, (x, y, z));
        }

        CreatureSpawnEventData {
            cancelled: true,
            ..event
        }
    }
}

/// The spawn reasons treated as the world spawning a mob on its own.
///
/// The API carries `spawn-reason` as a bare `string` with no documented vocabulary, so these
/// are the spellings observed to mean "natural". An unrecognised reason is **not** treated as
/// natural, and is logged once by [`note_reason`] so the real spelling can be seen in game
/// rather than guessed at.
const NATURAL_REASONS: [&str; 4] = ["natural", "chunk_generation", "chunkgeneration", "default"];

/// Returns whether `reason` means the world spawned this mob on its own.
///
/// A deliberate spawn — a spawner, a breeding pair, a spawn egg or a command — is left alone,
/// so cancelling here never removes a mob a player or operator asked for.
fn is_natural(reason: &str) -> bool {
    NATURAL_REASONS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(reason))
}

/// Logs a spawn reason the plugin does not recognise.
///
/// Without this the two failure modes are indistinguishable: a layer that spawns nothing
/// because no kind lives there looks exactly like a layer that spawns nothing because the
/// host words its reason differently. A debug line names the unexpected spellings, so the
/// real vocabulary can be read off a live server instead of guessed at.
///
/// Logged at `debug` rather than `warn`: this fires on every deliberate spawn in the world,
/// and an unexpected spelling is something to look up once, not a fault to report on every
/// tick of a busy server.
fn note_reason(reason: &str) {
    tracing::debug!("Unrecognised creature spawn reason '{reason}'; not replaced");
}

/// Picks the kind that spawns at `position` in `layer`, if the layer lists any.
///
/// The choice is derived from the position rather than from a random source, so it needs no
/// dependency and gives the same answer for the same spot twice — handy when a spawn looks
/// wrong in game. Only the kind list is consulted, never the world, so a layer with no
/// eligible kind costs one comparison and stops.
///
/// Villagers are excluded outright. A trader is placed where its shop stands, so a
/// naturally spawned one would be a shopkeeper with no counter and no map data behind it —
/// and a villager that never listed its layers would not be caught by the filter below.
fn choose(layer: crate::Layer, position: &(f64, f64, f64)) -> Option<MobKind> {
    let eligible: Vec<MobKind> = crate::mob_kinds()
        .into_iter()
        .filter(|kind| kind.spawns_in(layer) && kind.base != EntityType::Villager)
        .collect();

    if eligible.is_empty() {
        return None;
    }

    let (x, y, z) = *position;
    let mixed = x.to_bits().wrapping_mul(31) ^ y.to_bits() ^ z.to_bits().wrapping_mul(17);
    let index = usize::try_from(mixed % eligible.len() as u64).unwrap_or(0);

    eligible.get(index).copied()
}
