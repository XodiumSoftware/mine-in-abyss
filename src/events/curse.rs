//! The Curse of the Abyss.
//!
//! Ascending out of a layer inflicts symptoms that worsen with how deep the player
//! climbed from: a brief nausea near the surface, up to death from the Final
//! Maelstrom.

use pumpkin_plugin_api::damage_types_wit::DamageType;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::wit::pumpkin::plugin::status_effect::{
    StatusEffectInstance, StatusEffectType,
};

use crate::section::Section;

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

/// Applies the Curse of the Abyss to `player` after ascending out of `from`.
///
/// Severity comes from [`Section::curse`], so it scales with depth without a table
/// to keep in sync.
pub fn apply(player: &Player, from: Section) {
    match from.curse() {
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
