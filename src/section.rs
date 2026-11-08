//! The sections of the Abyss, from the surface down.

use crate::PLUGIN_ID;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A section of the Abyss.
///
/// Its [`From`](From::from) form is the lowercase variant name
/// (`first`, `second`, ...), which is also the identifier used for the section's
/// Minecraft world.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Section {
    /// The top of the Abyss.
    #[default]
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

impl Section {
    /// Returns this section's stable identifier (`first`, `second`, ...).
    ///
    /// This is the segment used to build the section's Minecraft world name. It is
    /// deliberately independent of any display name, so renaming a layer never
    /// invalidates an existing world on disk.
    #[must_use]
    pub fn id(self) -> &'static str {
        self.into()
    }

    /// Returns the name of the Minecraft world that holds this section.
    #[must_use]
    pub fn world(self) -> String {
        format!("{PLUGIN_ID}_{}", self.id())
    }

    /// Returns the section directly below this one, if any.
    #[must_use]
    pub fn next(self) -> Option<Self> {
        Self::iter().skip_while(|s| *s != self).nth(1)
    }

    /// Returns the section directly above this one, if any.
    #[must_use]
    pub fn prev(self) -> Option<Self> {
        let mut previous = None;
        for section in Self::iter() {
            if section == self {
                return previous;
            }
            previous = Some(section);
        }
        None
    }

    /// Returns this section's depth index, starting at 0 for the surface.
    #[must_use]
    pub fn index(self) -> usize {
        Self::iter()
            .position(|section| section == self)
            .unwrap_or(0)
    }

    /// Returns the severity of the Curse of the Abyss when ascending out of this
    /// section, from 0 (no ill effect) to 6 (fatal).
    ///
    /// Derived from the section's depth so it can never drift from the enum order.
    #[must_use]
    pub fn curse(self) -> u8 {
        u8::try_from(self.index()).unwrap_or(u8::MAX)
    }
}
