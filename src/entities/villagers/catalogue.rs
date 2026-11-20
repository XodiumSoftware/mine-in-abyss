//! Looking up a trader kind by name, for `/abyss spawn`.
//!
//! The traders are plain constructors with no catalogue, so this match is the only place a
//! name becomes one. Adding a trader means adding its arm here as well as its module.
//!
//! Names are matched case-insensitively against the trader's own display name, which includes
//! the title a trader carries (`Boy, Lord of Currency`), so the spelling has to stay identical
//! to the `TextComponent` its module builds.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::world::Entity;

/// A trader's spawn constructor, taking the world and a position.
pub(crate) type Spawn = fn(&World, (f64, f64, f64)) -> Entity;

/// Every trader name `/abyss spawn` accepts.
pub(crate) const NAMES: &[&str] = &[
    "alistair, geomancy enjoyer",
    "benner, also lord of currency",
    "berney, enchanter",
    "beebill, bee keeper",
    "bucketbill, bucket trader",
    "bilton, protector",
    "bob, handyman",
    "boy, lord of currency",
    "cedric, cementer",
    "cress, plant lover",
    "dayne, storage",
    "evril, void seeker",
    "frank, ocean fanatic",
    "gael, glass master",
    "hamilton, weaponsmith",
    "headmaster, the head trader",
    "heath, stone man",
    "issac, the intricate glazer",
    "jacob, archer",
    "jag, protector",
    "jeff, enchanter",
    "john, wheat dealer",
    "john, meat slinger",
    "larry, sculk lord",
    "mark, weaponsmith",
    "murdock, mudboy",
    "nereth, underworld merchant",
    "norah, the warped",
    "odroy, the apothecary",
    "pete, pothead",
    "quentin, keen weaver",
    "robert, enchanter",
    "rutrix, dj",
    "siggy, strider",
    "tags, weaponsmith",
    "veris, gem lover",
];

/// Returns the constructor for the trader called `name`, or `None` if no kind matches.
#[must_use]
pub(crate) fn spawn(name: &str) -> Option<Spawn> {
    match name.to_ascii_lowercase().as_str() {
        "alistair, geomancy enjoyer" => {
            Some(crate::entities::villagers::alistair::Alistair::entity)
        }
        "benner, also lord of currency" => Some(crate::entities::villagers::benner::Benner::entity),
        "berney, enchanter" => Some(crate::entities::villagers::berney::Berney::entity),
        "beebill, bee keeper" => Some(crate::entities::villagers::bill_bee::BeeBill::entity),
        "bucketbill, bucket trader" => {
            Some(crate::entities::villagers::bill_bucket::BucketBill::entity)
        }
        "bilton, protector" => Some(crate::entities::villagers::bilton::Bilton::entity),
        "bob, handyman" => Some(crate::entities::villagers::bob::Bob::entity),
        "boy, lord of currency" => Some(crate::entities::villagers::boy::Boy::entity),
        "cedric, cementer" => Some(crate::entities::villagers::cedric::Cedric::entity),
        "cress, plant lover" => Some(crate::entities::villagers::cress::Cress::entity),
        "dayne, storage" => Some(crate::entities::villagers::dayne::Dayne::entity),
        "evril, void seeker" => Some(crate::entities::villagers::evril::Evril::entity),
        "frank, ocean fanatic" => Some(crate::entities::villagers::frank::Frank::entity),
        "gael, glass master" => Some(crate::entities::villagers::gael::Gael::entity),
        "hamilton, weaponsmith" => Some(crate::entities::villagers::hamilton::Hamilton::entity),
        "headmaster, the head trader" => {
            Some(crate::entities::villagers::headmaster::Headmaster::entity)
        }
        "heath, stone man" => Some(crate::entities::villagers::heath::Heath::entity),
        "issac, the intricate glazer" => Some(crate::entities::villagers::issac::Issac::entity),
        "jacob, archer" => Some(crate::entities::villagers::jacob::Jacob::entity),
        "jag, protector" => Some(crate::entities::villagers::jag::Jag::entity),
        "jeff, enchanter" => Some(crate::entities::villagers::jeff::Jeff::entity),
        "john, wheat dealer" => Some(crate::entities::villagers::john::John::entity),
        "john, meat slinger" => Some(crate::entities::villagers::john_meat::MeatJohn::entity),
        "larry, sculk lord" => Some(crate::entities::villagers::larry::Larry::entity),
        "mark, weaponsmith" => Some(crate::entities::villagers::mark::Mark::entity),
        "murdock, mudboy" => Some(crate::entities::villagers::murdock::Murdock::entity),
        "nereth, underworld merchant" => Some(crate::entities::villagers::nereth::Nereth::entity),
        "norah, the warped" => Some(crate::entities::villagers::norah::Norah::entity),
        "odroy, the apothecary" => Some(crate::entities::villagers::odroy::Odroy::entity),
        "pete, pothead" => Some(crate::entities::villagers::pete::Pete::entity),
        "quentin, keen weaver" => Some(crate::entities::villagers::quentin::Quentin::entity),
        "robert, enchanter" => Some(crate::entities::villagers::robert::Robert::entity),
        "rutrix, dj" => Some(crate::entities::villagers::rutrix::Rutrix::entity),
        "siggy, strider" => Some(crate::entities::villagers::siggy::Siggy::entity),
        "tags, weaponsmith" => Some(crate::entities::villagers::tags::Tags::entity),
        "veris, gem lover" => Some(crate::entities::villagers::veris::Veris::entity),
        _ => None,
    }
}
