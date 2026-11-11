//! An example mob kind, showing what a custom mob can express.
//!
//! This is the worked example for [`Mob`](super::mob::Mob): it sets its attributes and its
//! equipment in one `apply`, which is how every kind defines itself — there is no fixed
//! table of multipliers or slots to fill in.
//!
//! In the Abyss this is a surface-layer scavenger: tougher than a plain zombie, hits harder,
//! wears a little armour and carries a sword, so a player can tell it apart from the vanilla
//! mob it is built on. The depth scale in
//! [`Monsters`](crate::events::monsters::Monsters) still applies on top, so the same kind is
//! far more dangerous six layers down.

use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::Mob;

/// A scavenger of the upper layers, built on a zombie.
#[derive(Debug, Default, Clone, Copy)]
pub struct Scavenger;

impl Mob for Scavenger {
    fn base() -> EntityType {
        EntityType::Zombie
    }

    fn id() -> u8 {
        1
    }

    fn name() -> &'static str {
        "Abyss Scavenger"
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
        entity.set_attribute_base(Attribute::AttackDamage, 6.0);
        entity.set_attribute_base(Attribute::FollowRange, 48.0);
        entity.set_attribute_base(Attribute::Armor, 4.0);
        entity.set_attribute_base(Attribute::MovementSpeed, 0.27);
    }
}
