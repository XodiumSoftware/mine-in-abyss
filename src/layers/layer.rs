//! Layer system for `mine-in-abyss`.
//!
//! The Abyss is a vertical world: the deeper you go, the more the rules change. This
//! module mirrors that. Every gameplay feature is a struct implementing the [`Layer`]
//! trait, and [`Depth`] names the strata it belongs to.
//!
//! Layers can register event handlers, commands, and permission nodes.

use crate::config::ConfigManager;
use crate::layers::fifth::FifthConfig;
use crate::layers::first::FirstConfig;
use crate::layers::fourth::FourthConfig;
use crate::layers::second::SecondConfig;
use crate::layers::seventh::SeventhConfig;
use crate::layers::sixth::SixthConfig;
use crate::layers::third::ThirdConfig;
use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::events::{EventHandler, EventPriority, FromIntoEvent};
use pumpkin_plugin_api::permission::Permission;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Player};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

/// Title fade-in time in ticks (half a second).
pub const TITLE_FADE_IN: i32 = 10;
/// How long the title stays on screen in ticks (two seconds).
pub const TITLE_STAY: i32 = 40;
/// Title fade-out time in ticks (half a second).
pub const TITLE_FADE_OUT: i32 = 10;

/// Parses a `#RRGGBB` (or `RRGGBB`) hex string into an [`RgbColor`].
///
/// Returns `None` for any other shape, so callers can fall back to a default.
#[must_use]
fn parse_hex(value: &str) -> Option<RgbColor> {
    let hex = value.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    Some(RgbColor {
        r: channel(0)?,
        g: channel(2)?,
        b: channel(4)?,
    })
}

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
    /// Returns the layer's effective display name, as configured.
    ///
    /// Reads the layer's `name` from config. Each layer defines that name as the
    /// default in its own `Default` impl, so there is a single source of truth; a
    /// blank or missing name falls back to the same default.
    #[must_use]
    pub fn name(self) -> String {
        ConfigManager::get()
            .map(|cm| cm.layers.config.name(self).to_owned())
            .filter(|name| !name.trim().is_empty())
            .unwrap_or_else(|| LayersConfig::default().name(self).to_owned())
    }

    /// Returns this stratum's name as a gradient-styled [`TextComponent`].
    ///
    /// The name from [`name`](Self::name), coloured with the palette from
    /// [`gradient_colors`](Self::gradient_colors).
    #[must_use]
    pub fn title(self) -> TextComponent {
        let colors = self.gradient_colors();
        TextComponent::text(&self.name()).gradient(&colors[..])
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
    /// Reads `gradient_start` / `gradient_end` from the layer's config, falling back to
    /// the layer's own defaults when the config is not loaded or a colour is malformed.
    #[must_use]
    pub fn gradient_colors(self) -> [RgbColor; 2] {
        let [start, end] = self.configured_gradient();
        let [default_start, default_end] = self.default_gradient();

        [
            parse_hex(&start).unwrap_or_else(|| {
                warn!("Invalid gradient_start '{start}' for {self:?}; using default");
                default_start
            }),
            parse_hex(&end).unwrap_or_else(|| {
                warn!("Invalid gradient_end '{end}' for {self:?}; using default");
                default_end
            }),
        ]
    }

    /// Returns the configured gradient hex strings.
    fn configured_gradient(self) -> [String; 2] {
        let config = ConfigManager::get().unwrap_or_default();
        [
            config.layers.config.gradient_start(self).to_owned(),
            config.layers.config.gradient_end(self).to_owned(),
        ]
    }

    /// Returns the layer's built-in gradient colours.
    fn default_gradient(self) -> [RgbColor; 2] {
        let config = LayersConfig::default();
        [
            parse_hex(config.gradient_start(self)).unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
            parse_hex(config.gradient_end(self)).unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
        ]
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
/// Each field holds one layer's own config, defined alongside that layer. Use
/// [`LayersConfig::is_enabled`], [`LayersConfig::name`], and the gradient accessors to
/// look a layer up from its stratum instead of matching on the fields directly.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LayersConfig {
    /// The First Layer (Y 195–319).
    pub first: FirstConfig,
    /// The Second Layer (Y 126–194).
    pub second: SecondConfig,
    /// The Third Layer (Y 66–125).
    pub third: ThirdConfig,
    /// The Fourth Layer (Y 6–65).
    pub fourth: FourthConfig,
    /// The Fifth Layer (Y -24 to 5).
    pub fifth: FifthConfig,
    /// The Sixth Layer (Y -50 to -25).
    pub sixth: SixthConfig,
    /// The Seventh Layer (Y -64 to -51).
    pub seventh: SeventhConfig,
}

impl LayersConfig {
    /// Returns whether the layer for `depth` is enabled.
    #[must_use]
    pub const fn is_enabled(&self, depth: Depth) -> bool {
        match depth {
            Depth::First => self.first.enabled,
            Depth::Second => self.second.enabled,
            Depth::Third => self.third.enabled,
            Depth::Fourth => self.fourth.enabled,
            Depth::Fifth => self.fifth.enabled,
            Depth::Sixth => self.sixth.enabled,
            Depth::Seventh => self.seventh.enabled,
        }
    }

    /// Returns the configured display name for `depth`.
    #[must_use]
    pub fn name(&self, depth: Depth) -> &str {
        match depth {
            Depth::First => &self.first.name,
            Depth::Second => &self.second.name,
            Depth::Third => &self.third.name,
            Depth::Fourth => &self.fourth.name,
            Depth::Fifth => &self.fifth.name,
            Depth::Sixth => &self.sixth.name,
            Depth::Seventh => &self.seventh.name,
        }
    }

    /// Returns the configured gradient start colour for `depth`.
    #[must_use]
    pub fn gradient_start(&self, depth: Depth) -> &str {
        match depth {
            Depth::First => &self.first.gradient_start,
            Depth::Second => &self.second.gradient_start,
            Depth::Third => &self.third.gradient_start,
            Depth::Fourth => &self.fourth.gradient_start,
            Depth::Fifth => &self.fifth.gradient_start,
            Depth::Sixth => &self.sixth.gradient_start,
            Depth::Seventh => &self.seventh.gradient_start,
        }
    }

    /// Returns the configured gradient end colour for `depth`.
    #[must_use]
    pub fn gradient_end(&self, depth: Depth) -> &str {
        match depth {
            Depth::First => &self.first.gradient_end,
            Depth::Second => &self.second.gradient_end,
            Depth::Third => &self.third.gradient_end,
            Depth::Fourth => &self.fourth.gradient_end,
            Depth::Fifth => &self.fifth.gradient_end,
            Depth::Sixth => &self.sixth.gradient_end,
            Depth::Seventh => &self.seventh.gradient_end,
        }
    }
}
