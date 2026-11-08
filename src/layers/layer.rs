//! Layer system for `mine-in-abyss`.
//!
//! The Abyss is a vertical world: the deeper you go, the more the rules change. This
//! module mirrors that. Every gameplay feature is a struct implementing the [`Layer`]
//! trait, and [`Depth`] names the strata it belongs to.
//!
//! Layers can register event handlers, commands, and permission nodes.

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use serde::{Deserialize, Serialize};
use tracing::error;

/// The known strata of the Abyss, from the surface down.
///
/// A layer's [`Depth`] is purely descriptive: it documents where a feature sits in
/// the world and gives the plugin one place to reason about depth ranges.
///
/// Ranges are expressed as Minecraft world Y coordinates, spanning the full playable
/// column from the world floor (Y -64) up to the build limit (Y 319).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Depth {
    /// The top of the Abyss (Y 195–319).
    #[default]
    First,
    /// The Forest of Temptation (Y 126–194).
    Second,
    /// The Great Fault (Y 66–125).
    Third,
    /// The Goblets of Giants (Y 6–65).
    Fourth,
    /// The Sea of Corpses (Y -24 to 5).
    Fifth,
    /// The Capital of the Unreturned (Y -50 to -25).
    Sixth,
    /// The Final Maelstrom (Y -64 to -51).
    Seventh,
}

impl Depth {
    /// Every stratum, in order from the surface down.
    pub const ALL: [Self; 7] = [
        Self::First,
        Self::Second,
        Self::Third,
        Self::Fourth,
        Self::Fifth,
        Self::Sixth,
        Self::Seventh,
    ];
    /// Returns the human-readable name of this stratum.
    ///
    /// This is the display/identity name (e.g. `"The Forest of Temptation"`), as
    /// opposed to the ordinal variant name used for serialization.
    ///
    /// [`Depth::First`] has no separate name in the source material, so it returns
    /// a descriptive fallback of `"The Top of the Abyss"`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::First => "The Top of the Abyss",
            Self::Second => "The Forest of Temptation",
            Self::Third => "The Great Fault",
            Self::Fourth => "The Goblets of Giants",
            Self::Fifth => "The Sea of Corpses",
            Self::Sixth => "The Capital of the Unreturned",
            Self::Seventh => "The Final Maelstrom",
        }
    }

    /// Returns the `MiniMessage` colour tag used to render this stratum's name.
    ///
    /// This is a raw, unparsed `MiniMessage` tag (a gradient between two hex colours),
    /// not rendered text. Pair it with [`name`](Self::name) — or use
    /// [`colored_name`](Self::colored_name) — and let the renderer turn it into a
    /// component.
    ///
    /// The palette runs from warm daylight at the surface to cold, lightless colours
    /// in the depths.
    #[must_use]
    pub const fn color(self) -> &'static str {
        match self {
            Self::First => "<gradient:#87CEEB:#5BA85B>",
            Self::Second => "<gradient:#5BA85B:#2E8B57>",
            Self::Third => "<gradient:#8B7355:#6B5B4A>",
            Self::Fourth => "<gradient:#6B5B4A:#4A3F5C>",
            Self::Fifth => "<gradient:#4A3F5C:#2E2A45>",
            Self::Sixth => "<gradient:#2E2A45:#1A1728>",
            Self::Seventh => "<gradient:#1A1728:#6B0000>",
        }
    }

    /// Returns this stratum's name wrapped in its `MiniMessage` colour tag.
    ///
    /// The result is still unparsed `MiniMessage`, ready to be handed to a `MiniMessage`
    /// renderer: the tag from [`color`](Self::color), then the name from
    /// [`name`](Self::name), then a closing gradient tag.
    #[must_use]
    pub fn colored_name(self) -> String {
        format!("{}{}</gradient>", self.color(), self.name())
    }

    /// Returns the name of the Minecraft world that holds this stratum.
    ///
    /// The world name is a valid, filesystem-safe identifier: the plugin id
    /// followed by the stratum's ordinal (`mine_in_abyss_first`, ...). It is
    /// stable, so changing a layer's display [`name`](Self::name) never
    /// invalidates an existing world on disk.
    #[must_use]
    pub const fn world_name(self) -> &'static str {
        match self {
            Self::First => "mine_in_abyss_first",
            Self::Second => "mine_in_abyss_second",
            Self::Third => "mine_in_abyss_third",
            Self::Fourth => "mine_in_abyss_fourth",
            Self::Fifth => "mine_in_abyss_fifth",
            Self::Sixth => "mine_in_abyss_sixth",
            Self::Seventh => "mine_in_abyss_seventh",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Depth;

    const ALL: [Depth; 7] = [
        Depth::First,
        Depth::Second,
        Depth::Third,
        Depth::Fourth,
        Depth::Fifth,
        Depth::Sixth,
        Depth::Seventh,
    ];

    #[test]
    fn every_stratum_has_a_name() {
        for depth in ALL {
            assert!(!depth.name().is_empty(), "{depth:?} has an empty name");
        }
    }

    #[test]
    fn names_are_unique() {
        for (i, a) in ALL.iter().enumerate() {
            for b in &ALL[i + 1..] {
                assert_ne!(a.name(), b.name(), "{a:?} and {b:?} share a name");
            }
        }
    }

    #[test]
    fn every_stratum_has_a_color_tag() {
        for depth in ALL {
            let color = depth.color();
            assert!(
                color.starts_with("<gradient:#") && color.ends_with('>'),
                "{depth:?} has a malformed colour tag: {color}"
            );
        }
    }

    #[test]
    fn colored_name_wraps_the_name_in_its_tag() {
        for depth in ALL {
            let colored = depth.colored_name();
            assert!(
                colored.starts_with(depth.color()),
                "{depth:?} does not start with its colour tag"
            );
            assert!(
                colored.contains(depth.name()),
                "{depth:?} coloured name is missing the plain name"
            );
            assert!(
                colored.ends_with("</gradient>"),
                "{depth:?} coloured name is not closed"
            );
        }
    }
}

/// A trait representing a plugin layer that can be enabled or disabled.
///
/// Layers may optionally expose commands, permission nodes, and event handlers,
/// all registered with the server via [`Layer::register`].
pub trait Layer {
    /// Returns the stratum of the Abyss this layer belongs to.
    fn depth(&self) -> Depth {
        Depth::First
    }

    /// Returns `true` if the layer is enabled, `false` otherwise.
    fn enabled(&self) -> bool;

    /// Returns the commands provided by this layer.
    ///
    /// Each [`Command`] returned here will be registered with the server when
    /// [`Layer::register`] is called. Returns an empty vec by default.
    fn cmds(&self) -> Vec<Command> {
        vec![]
    }

    /// Returns the permission nodes required by this layer.
    ///
    /// All permissions returned here are registered with the server so permission
    /// plugins can grant/deny them. Enforcement happens inside each command's
    /// `CommandHandler` via `sender.has_permission(server, node)`.
    /// Returns an empty vec by default.
    fn perms(&self) -> Vec<Permission> {
        Vec::new()
    }

    /// Registers event handlers for this layer.
    ///
    /// Override this to call [`Layer::register_event`] for each event this layer
    /// handles. No-op by default.
    fn events(&self, _context: &Context) {}

    /// Registers `Self` as the handler for event `T` and panics on failure.
    ///
    /// This is a thin wrapper around [`Context::register_event_handler`] that
    /// supplies the layer-specific error message so individual layers don't
    /// have to repeat it.
    fn register_event<T>(&self, context: &Context, priority: EventPriority, ignore_cancelled: bool)
    where
        T: FromIntoEvent + Send + Sync + 'static,
        Self: EventHandler<T> + Default + Send + Sync + 'static,
    {
        context
            .register_event_handler::<T, _>(Self::default(), priority, ignore_cancelled)
            .expect("failed to register event handler");
    }

    /// Registers this layer's event handlers, permissions, and commands with the server.
    ///
    /// No-op when [`Layer::enabled`] returns `false`.
    fn register(&self, context: &Context) {
        if !self.enabled() {
            return;
        }
        self.events(context);
        for perm in self.perms() {
            if let Err(e) = context.register_permission(&perm) {
                error!("Failed to register permission '{}': {e}", perm.node);
            }
        }
        for cmd in self.cmds() {
            context.register_command(cmd, "");
        }
    }
}

/// Top-level configuration for all layers (stored in `layers.toml`).
///
/// Each field toggles one layer. Add new layers here, grouped by the [`Depth`]
/// they belong to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LayersConfig {}
