//! Keeps every layer world on the same clock.
//!
//! Every world keeps time independently, so left alone they drift apart and a descending
//! player would watch the sun jump. One world is authoritative and the rest follow it on a
//! slow repeating task.
//!
//! The period is slow because setting the time of day makes the world tell its clients about
//! it; a minute is imperceptible and keeps that chatter off the tick budget. The task also
//! runs once immediately on load, so a player arriving after a restart does not see a stale
//! sun.
//!
//! The whole `time_of_day` value is copied, not just the sub-day part, so the **day number**
//! follows too. A world's **age** is a separate counter with no setter in the API, so it
//! cannot be shared — nothing in vanilla depends on the two agreeing.

use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::{Context, Server};

use crate::LAYERS;
use crate::layer::Layer;
use crate::utils::world_for;

/// The layer whose clock every other layer follows: the city above the Abyss, and the layer
/// an operator is most likely to set the time in.
const SOURCE: Layer = Layer::Orth;

/// Ticks between two synchronisations: one minute at 20 ticks per second.
const SYNC_PERIOD: u64 = 20 * 60;

/// Starts the repeating task that keeps the layer clocks together.
///
/// Scheduled through `context` because `Server` is a non-cloneable resource and cannot be
/// captured by the task closure.
pub(crate) fn schedule(context: &Context) -> u32 {
    context.schedule_repeating_task(1, SYNC_PERIOD, |server| sync_times(&server))
}

/// Copies the source layer's clock to every other layer world.
fn sync_times(server: &Server) {
    let Some(source) = world_for(server, SOURCE, "its clock cannot be synced") else {
        return;
    };

    let time = source.get_time_of_day();

    for layer in LAYERS {
        if layer == SOURCE {
            continue;
        }

        if let Some(world) = world_for(server, layer, "its clock cannot be synced") {
            world.set_time_of_day(time);
        }
    }
}
