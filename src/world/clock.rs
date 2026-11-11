//! Keeps every layer world on the same clock.
//!
//! Each layer is its own Minecraft world, and every world keeps time independently. Left
//! alone they drift apart, so a player who descends would arrive at a different time of
//! day than the one they left — the sun would jump mid-fall, and the day count would
//! disagree. This module copies the clock from one source world to the rest on a slow
//! repeating task.
//!
//! # Why a source world rather than a computed time
//!
//! The layers have no shared clock, so one of them has to be authoritative. `First` is
//! that source: it is the surface, the layer an operator is most likely to change the time
//! in, and the one every descent starts from.
//!
//! # Why the period is slow
//!
//! Reading and writing the time of day is cheap, but setting it forces the world to tell
//! its clients about the change. A minute is far below the granularity anyone can perceive
//! and keeps that chatter off the tick budget. The cost of a slow period is that two layers
//! can disagree by up to one period, which is why the task runs once immediately on load —
//! so a player arriving right after a restart does not see a stale sun.
//!
//! # What is and is not shared
//!
//! The layers share the value behind `time_of_day`, which Minecraft derives both the time of
//! day and the **day number** from (`time / 24000`). Copying it whole, rather than only the
//! sub-day part, is what keeps the two layers on the same day as well as the same hour.
//!
//! A world's **age** is a separate counter, and the plugin API exposes no setter for it, so
//! age cannot be shared. Nothing in vanilla depends on a world's age matching its time of
//! day, and `/time set` desynchronises the two in vanilla as well.

use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::{Context, Server};

use crate::LAYERS;
use crate::layer::Layer;
use crate::utils::world_for;

/// The layer whose clock every other layer follows.
const SOURCE: Layer = Layer::First;

/// Ticks between two synchronisations: one minute at 20 ticks per second.
///
/// Slow enough to be invisible in cost, and still far quicker than the time of day can
/// meaningfully change.
const SYNC_PERIOD: u64 = 20 * 60;

/// Starts the repeating task that keeps the layer clocks together.
///
/// Returns the task id, so it can be cancelled when the plugin unloads.
///
/// Scheduled through `context` rather than the server: `Server` is a non-cloneable resource,
/// so it cannot be captured by the task closure, and [`SchedulerExt`] offers the same call
/// on [`Context`](pumpkin_plugin_api::Context).
pub fn schedule(context: &Context) -> u32 {
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
