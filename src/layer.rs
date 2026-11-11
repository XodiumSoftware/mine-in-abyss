//! The layers of the Abyss, from the surface down.

use crate::PLUGIN_ID;
use crate::utils::rgb;
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::{GameRule, GameRuleValue};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A layer of the Abyss.
///
/// Its [`From`](From::from) form is the lowercase variant name
/// (`orth`, `first`, ...), which is also the identifier used for the layer's
/// Minecraft world.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Layer {
    /// The city on the edge, above the Abyss itself.
    #[default]
    Orth,
    /// The Depths of the Abyss.
    First,
    /// The Forest of Temptation.
    Second,
    /// The Great Fault.
    Third,
    /// The Goblets of Giants.
    Fourth,
    /// The Sea of Corpses.
    Fifth,
    /// The Capital of the Unreturned.
    Sixth,
    /// The Final Maelstrom.
    Seventh,
}

impl Layer {
    /// Every layer, from the surface down.
    pub const ALL: [Self; 8] = [
        Self::Orth,
        Self::First,
        Self::Second,
        Self::Third,
        Self::Fourth,
        Self::Fifth,
        Self::Sixth,
        Self::Seventh,
    ];

    /// The world floor, and the Y coordinate below which a player has left the layer.
    ///
    /// Every layer is its own full-height Overworld-dimension world, so this is `-64`.
    /// Falling past it hands the player to vanilla void damage.
    pub const MIN_Y: f64 = -64.0;

    /// Y coordinate above which a player is considered to have left through the ceiling.
    ///
    /// The build limit is 319, so this is just above it.
    pub const MAX_Y: f64 = 320.0;

    /// Returns this layer's stable identifier (`first`, `second`, ...).
    ///
    /// This is the segment used to build the layer's Minecraft world name. It is
    /// deliberately independent of any display name, so renaming a layer never
    /// invalidates an existing world on disk.
    #[must_use]
    pub fn id(self) -> &'static str {
        self.into()
    }

    /// Returns the name of the Minecraft world that holds this layer.
    #[must_use]
    pub fn world(self) -> String {
        format!("{PLUGIN_ID}_{}", self.id())
    }

    /// Returns this layer's display name, as shown to players.
    ///
    /// This is the single source of truth for the name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Orth => "The City on the Edge",
            Self::First => "Depths of the Abyss",
            Self::Second => "The Forest of Temptation",
            Self::Third => "The Great Fault",
            Self::Fourth => "The Goblets of Giants",
            Self::Fifth => "The Sea of Corpses",
            Self::Sixth => "The Capital of the Unreturned",
            Self::Seventh => "The Final Maelstrom",
        }
    }

    /// Returns the two gradient stop colours of this layer's palette.
    #[must_use]
    pub const fn color(self) -> [RgbColor; 2] {
        match self {
            Self::Orth => [rgb(0xB0, 0xE0, 0xF0), rgb(0x87, 0xCE, 0xEB)],
            Self::First => [rgb(0x87, 0xCE, 0xEB), rgb(0x5B, 0xA8, 0x5B)],
            Self::Second => [rgb(0x5B, 0xA8, 0x5B), rgb(0x2E, 0x8B, 0x57)],
            Self::Third => [rgb(0x8B, 0x73, 0x55), rgb(0x6B, 0x5B, 0x4A)],
            Self::Fourth => [rgb(0x6B, 0x5B, 0x4A), rgb(0x4A, 0x3F, 0x5C)],
            Self::Fifth => [rgb(0x4A, 0x3F, 0x5C), rgb(0x2E, 0x2A, 0x45)],
            Self::Sixth => [rgb(0x2E, 0x2A, 0x45), rgb(0x1A, 0x17, 0x28)],
            Self::Seventh => [rgb(0x1A, 0x17, 0x28), rgb(0x6B, 0x00, 0x00)],
        }
    }

    /// Returns the game rules this layer's world runs with.
    ///
    /// Applied by [`Worlds::setup`](crate::Worlds::setup) whenever the world is created or
    /// re-attached, so the rules win over whatever the world folder was saved with. This is
    /// per-world state: Minecraft's *difficulty* is a server-wide setting with no setter in
    /// the plugin API, so peaceful behaviour is expressed as game rules instead.
    #[must_use]
    pub const fn game_rules(self) -> &'static [(GameRule, GameRuleValue)] {
        match self {
            Self::Orth | Self::First => &[(GameRule::SpawnMonsters, GameRuleValue::Bool(false))],
            Self::Second
            | Self::Third
            | Self::Fourth
            | Self::Fifth
            | Self::Sixth
            | Self::Seventh => &[],
        }
    }

    /// Returns the layer directly below this one, if any.
    #[must_use]
    pub fn next(self) -> Option<Self> {
        Self::iter().skip_while(|s| *s != self).nth(1)
    }

    /// Returns the layer directly above this one, if any.
    #[must_use]
    pub fn prev(self) -> Option<Self> {
        Self::iter().take_while(|layer| *layer != self).last()
    }

    /// Returns this layer's depth index, starting at 0 for the surface.
    #[must_use]
    pub fn index(self) -> usize {
        Self::iter().position(|layer| layer == self).unwrap_or(0)
    }

    /// Returns this layer's one-based number (`1` for the surface, `8` at the
    /// bottom), as shown to players.
    #[must_use]
    pub fn number(self) -> usize {
        self.index() + 1
    }

    /// Resolves a user-supplied identifier (`first`, `1`, `FIRST`, …) to a layer.
    ///
    /// Accepts both the [`id`](Self::id) and the [`number`](Self::number), case-insensitively,
    /// so a command can take whichever the player finds easier to type. Both spellings are
    /// read from the variant itself, so a renamed or reordered layer cannot leave a stale
    /// alias behind.
    #[must_use]
    pub fn from_id(input: &str) -> Option<Self> {
        let input = input.to_ascii_lowercase();
        Self::iter().find(|layer| layer.id() == input || layer.number().to_string() == input)
    }

    /// Returns every identifier, comma-separated, for use in error messages.
    #[must_use]
    pub fn id_list() -> String {
        Self::iter().map(Self::id).collect::<Vec<_>>().join(", ")
    }

    /// Returns the Y coordinate a player is placed at when entering a layer.
    ///
    /// Entering from above (descending) lands them near the ceiling of the world, so the
    /// fall into the layer is visible. This is the same for every layer: each is its own
    /// full-height world, so the height comes from the shared world limits rather than from
    /// the layer's depth.
    #[must_use]
    pub const fn entry_y() -> f64 {
        Self::MAX_Y - 2.0
    }

    /// Returns the Y coordinate a player is placed at when leaving a layer upward.
    ///
    /// They arrive just above the floor, inside the void band, so they climb out into the
    /// layer rather than falling through it again. Like [`entry_y`](Self::entry_y), this is
    /// the same for every layer.
    #[must_use]
    pub const fn exit_y() -> f64 {
        Self::MIN_Y + 1.0
    }

    /// Returns the severity of the Curse of the Abyss when ascending out of this layer,
    /// from 0 (no ill effect) to 6 (fatal).
    ///
    /// Derived from the layer's depth so it can never drift from the enum order.
    #[must_use]
    pub fn curse(self) -> u8 {
        u8::try_from(self.index()).unwrap_or(u8::MAX)
    }
}
