//! The Curse of the Abyss.
//!
//! Ascending out of a layer inflicts symptoms that worsen with how deep the player
//! climbed from: a brief nausea near the surface, up to death from the Final
//! Maelstrom.
//!
//! The curse is **cumulative**: each ascent adds strain rather than replacing the
//! previous symptoms, so climbing back up from several layers is worse than climbing
//! up one. The strain is stored on the player as persistent data, so it survives a
//! relog, and only reaching the surface — or dying — clears it.

use pumpkin_plugin_api::damage_types_wit::DamageType;
use pumpkin_plugin_api::persistent_data::PersistentDataHolder;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::wit::pumpkin::plugin::status_effect::{
    StatusEffectInstance, StatusEffectType,
};

use crate::PLUGIN_ID;
use crate::layer::Layer;

/// Namespace key under which a player's accumulated curse strain is stored.
const STRAIN_KEY: &str = "curse";

/// Highest strain a player can carry.
///
/// Matches the deepest layer's [`Layer::curse`] severity, so strain saturates
/// at the fatal tier rather than growing without bound.
const MAX_STRAIN: u8 = 6;

/// Builds a status effect instance, showing particles and icon by default.
fn effect(kind: StatusEffectType, seconds: u32, amplifier: u8) -> StatusEffectInstance {
    StatusEffectInstance {
        effect_type: kind,
        duration: seconds * 20,
        amplifier,
        ambient: false,
        show_particles: true,
        show_icon: true,
    }
}

/// Returns the curse strain the player is currently carrying, `0` if unafflicted.
#[must_use]
pub fn strain(player: &Player) -> u8 {
    player
        .get_byte(PLUGIN_ID, STRAIN_KEY)
        .and_then(|strain| u8::try_from(strain).ok())
        .unwrap_or(0)
}

/// Stores the player's curse strain, clearing it when `strain` is `0`.
fn store(player: &Player, strain: u8) {
    if strain == 0 {
        player.remove_custom_data(PLUGIN_ID, STRAIN_KEY);
    } else if let Ok(strain) = i8::try_from(strain) {
        player.set_byte(PLUGIN_ID, STRAIN_KEY, strain);
    }
}

/// Applies the Curse of the Abyss to `player` after ascending out of `from`.
///
/// The severity comes from [`Layer::curse`], so it scales with depth without a
/// table to keep in sync. It is added to the player's accumulated strain, which is
/// what actually decides the symptoms.
pub fn apply(player: &Player, from: Layer) {
    let strain = strain(player).saturating_add(from.curse()).min(MAX_STRAIN);
    store(player, strain);
    inflict(player, strain);
}

/// Clears the player's accumulated curse strain.
///
/// Called when a player reaches the surface, and when they respawn, so a death never
/// leaves a permanent debuff behind.
pub fn cure(player: &Player) {
    store(player, 0);
}

/// Inflicts the symptoms of a given total `strain` on `player`.
fn inflict(player: &Player, strain: u8) {
    match strain {
        0 => {}
        1 => {
            player.add_effect(effect(StatusEffectType::Nausea, 5, 0));
        }
        2 => {
            player.add_effect(effect(StatusEffectType::Nausea, 8, 1));
            player.add_effect(effect(StatusEffectType::Weakness, 5, 0));
        }
        3 => {
            player.add_effect(effect(StatusEffectType::Nausea, 12, 2));
            player.add_effect(effect(StatusEffectType::Blindness, 2, 0));
            player.add_effect(effect(StatusEffectType::Slowness, 8, 1));
        }
        4 => {
            player.add_effect(effect(StatusEffectType::Nausea, 15, 2));
            player.add_effect(effect(StatusEffectType::Blindness, 4, 0));
            player.add_effect(effect(StatusEffectType::Weakness, 12, 0));
            player.damage(4.0, DamageType::Magic);
        }
        5 => {
            player.add_effect(effect(StatusEffectType::Blindness, 8, 0));
            player.add_effect(effect(StatusEffectType::Weakness, 20, 2));
            player.damage(8.0, DamageType::Magic);
            player.set_health(player.get_health().min(1.0));
        }
        _ => {
            player.kill();
        }
    }
}
