//! Layer system for `mine-in-abyss`.
//!
//! The Abyss is a vertical world: the deeper you go, the more the rules change. This
//! module mirrors that. Every gameplay feature is a struct implementing the [`Layer`]
//! trait, and [`Depth`] names the strata it belongs to.
//!
//! Layers can register event handlers, commands, and permission nodes.

use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Player};
use serde::{Deserialize, Serialize};
use tracing::error;

/// Title fade-in time in ticks (half a second).
pub const TITLE_FADE_IN: i32 = 10;
/// How long the title stays on screen in ticks (two seconds).
pub const TITLE_STAY: i32 = 40;
/// Title fade-out time in ticks (half a second).
pub const TITLE_FADE_OUT: i32 = 10;

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

    /// Returns this stratum's name as a gradient-styled [`TextComponent`].
    ///
    /// The name from [`name`](Self::name), coloured with the palette from
    /// [`gradient_colors`](Self::gradient_colors).
    #[must_use]
    pub fn title(self) -> TextComponent {
        let colors = self.gradient_colors();
        TextComponent::text(self.name()).gradient(&colors[..])
    }

    /// Shows the "entered this layer" title to a player.
    ///
    /// The title is [`Depth::title`] and the subtitle is a short `Entered <name>`
    /// line. Shared by every layer so the presentation stays consistent.
    pub fn announce_entry(self, player: &Player) {
        player.show_title(self.title());
        player.show_subtitle(TextComponent::text(&format!("Entered {}", self.name())));
        player.send_title_animation(TITLE_FADE_IN, TITLE_STAY, TITLE_FADE_OUT);
    }

    /// Returns the gradient stop colours of this stratum's palette.
    ///
    /// The palette runs from warm daylight at the surface to cold, lightless colours
    /// in the depths.
    #[must_use]
    pub fn gradient_colors(self) -> [RgbColor; 2] {
        match self {
            Self::First => [
                RgbColor {
                    r: 0x87,
                    g: 0xCE,
                    b: 0xEB,
                },
                RgbColor {
                    r: 0x5B,
                    g: 0xA8,
                    b: 0x5B,
                },
            ],
            Self::Second => [
                RgbColor {
                    r: 0x5B,
                    g: 0xA8,
                    b: 0x5B,
                },
                RgbColor {
                    r: 0x2E,
                    g: 0x8B,
                    b: 0x57,
                },
            ],
            Self::Third => [
                RgbColor {
                    r: 0x8B,
                    g: 0x73,
                    b: 0x55,
                },
                RgbColor {
                    r: 0x6B,
                    g: 0x5B,
                    b: 0x4A,
                },
            ],
            Self::Fourth => [
                RgbColor {
                    r: 0x6B,
                    g: 0x5B,
                    b: 0x4A,
                },
                RgbColor {
                    r: 0x4A,
                    g: 0x3F,
                    b: 0x5C,
                },
            ],
            Self::Fifth => [
                RgbColor {
                    r: 0x4A,
                    g: 0x3F,
                    b: 0x5C,
                },
                RgbColor {
                    r: 0x2E,
                    g: 0x2A,
                    b: 0x45,
                },
            ],
            Self::Sixth => [
                RgbColor {
                    r: 0x2E,
                    g: 0x2A,
                    b: 0x45,
                },
                RgbColor {
                    r: 0x1A,
                    g: 0x17,
                    b: 0x28,
                },
            ],
            Self::Seventh => [
                RgbColor {
                    r: 0x1A,
                    g: 0x17,
                    b: 0x28,
                },
                RgbColor {
                    r: 0x6B,
                    g: 0x00,
                    b: 0x00,
                },
            ],
        }
    }

    /// Returns the stratum whose world is called `name`, if any.
    ///
    /// This is the inverse of [`world_name`](Self::world_name).
    #[must_use]
    pub fn from_world_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|depth| depth.world_name() == name)
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

/// Configuration for every layer, keyed by stratum.
///
/// Each field configures one layer. Add new layers here, grouped by the
/// [`Depth`] they belong to. Use [`LayersConfig::for_depth`] to look a layer up
/// from its stratum instead of matching on the fields directly.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LayersConfig {
    /// The First Layer (Y 195–319).
    pub first: LayerConfig,
    /// The Second Layer (Y 126–194).
    pub second: LayerConfig,
    /// The Third Layer (Y 66–125).
    pub third: LayerConfig,
    /// The Fourth Layer (Y 6–65).
    pub fourth: LayerConfig,
    /// The Fifth Layer (Y -24 to 5).
    pub fifth: LayerConfig,
    /// The Sixth Layer (Y -50 to -25).
    pub sixth: LayerConfig,
    /// The Seventh Layer (Y -64 to -51).
    pub seventh: LayerConfig,
}

impl LayersConfig {
    /// Returns the configuration for `depth`.
    #[must_use]
    pub const fn for_depth(&self, depth: Depth) -> &LayerConfig {
        match depth {
            Depth::First => &self.first,
            Depth::Second => &self.second,
            Depth::Third => &self.third,
            Depth::Fourth => &self.fourth,
            Depth::Fifth => &self.fifth,
            Depth::Sixth => &self.sixth,
            Depth::Seventh => &self.seventh,
        }
    }

    /// Returns whether the layer for `depth` is enabled.
    #[must_use]
    pub const fn is_enabled(&self, depth: Depth) -> bool {
        self.for_depth(depth).enabled
    }
}

/// Configuration for a single layer.
///
/// Shared by every stratum; add per-layer fields here as features land.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LayerConfig {
    /// Whether this layer is active.
    pub enabled: bool,
}
