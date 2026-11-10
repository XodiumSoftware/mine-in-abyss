//! The `/abyss` command.
//!
//! Moves a player into any layer of the Abyss directly, without ascending through the
//! Abyss in between, so the Curse of the Abyss is never inflicted.
//!
//! # Usage
//!
//! ```text
//! /abyss <layer> [player]
//! ```
//!
//! - `<layer>` — `first` … `seventh`, or `1` … `7`, case-insensitive
//! - `[player]` — who to move; only used when the command comes from **console**, since a
//!   player sender is always moved themselves
//!
//! Tab-completion for `<layer>` is implemented but disabled: the Pumpkin host panics on
//! any suggestion request (see [`LayerSuggestions`]).
//!
//! # Permissions
//!
//! | Node                          | Default                    |
//! | ----------------------------- | -------------------------- |
//! | `mine_in_abyss:command.abyss` | granted to ops at level 3+ |

use pumpkin_plugin_api::command::{
    Arg, ArgumentType, Command, CommandError, CommandNode, CommandSender, ConsumedArgs, StringType,
};
#[cfg(any())]
use pumpkin_plugin_api::command::{CommandSuggestion, CommandSuggestions, SuggestionRequest};
use pumpkin_plugin_api::commands::CommandHandler;
#[cfg(any())]
use pumpkin_plugin_api::commands::CommandSuggestionHandler;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::permission::{
    Permission, PermissionDefault, PermissionLevel,
};
use pumpkin_plugin_api::{Context, Server};

use crate::PLUGIN_ID;
use crate::layer::Layer;

/// The command's primary name followed by its aliases.
const NAMES: [&str; 2] = ["abyss", "mia"];

/// Permission node that gates the command.
///
/// Built from [`crate::PLUGIN_ID`] so the namespace never drifts from the crate's.
pub fn permission_node() -> String {
    format!("{PLUGIN_ID}:command.abyss")
}

/// Human-readable description, reused by the command and its permission node.
const DESCRIPTION: &str = "Move to a layer of the Abyss without the curse";

/// Registers `/abyss` and its permission node.
pub fn register(context: &Context) {
    let names: Vec<String> = NAMES.iter().map(ToString::to_string).collect();

    let layer_leaf = CommandNode::argument("layer", &ArgumentType::String(StringType::SingleWord));
    let layer_with_player =
        CommandNode::argument("layer", &ArgumentType::String(StringType::SingleWord));
    let player = CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord));

    let command = Command::new(&names, DESCRIPTION)
        .then(layer_with_player.then(player).execute(AbyssCommand))
        .then(layer_leaf.execute(AbyssCommand))
        .execute(AbyssCommand);

    context.register_command(command, &permission_node());

    let _ = context.register_permission(&Permission {
        node: permission_node(),
        description: DESCRIPTION.to_string(),
        default: PermissionDefault::Op(PermissionLevel::Three),
        children: vec![],
    });
}

/// Executor for `/abyss`.
#[derive(Debug, Default, Clone, Copy)]
struct AbyssCommand;

impl CommandHandler for AbyssCommand {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let Arg::Simple(layer) = args.get_value("layer") else {
            return Err(CommandError::InvalidConsumption(None));
        };

        let Some(layer) = layer_from(&layer) else {
            sender.send_error(TextComponent::text(&format!(
                "Unknown layer '{layer}'. Expected one of: {}.",
                layer_names()
            )));
            return Ok(0);
        };

        let target = match sender.as_player() {
            Some(player) => Some(player),
            None => match args.get_value("player") {
                Arg::Simple(name) if !name.is_empty() => server.get_player_by_name(&name),
                _ => None,
            },
        };

        let Some(player) = target else {
            sender.send_error(TextComponent::text("Console must name the player to move."));
            return Ok(0);
        };

        let Some(world) = server.get_world_by_name(&layer.world()) else {
            sender.send_error(TextComponent::text(&format!(
                "The world for {} is not available.",
                layer.id()
            )));
            return Ok(0);
        };

        let (x, _, z) = player.get_position();
        player.teleport((x, Layer::entry_y(), z), None, None, world);

        sender.send_message(TextComponent::text(&format!(
            "Moved {} to {}.",
            player.get_name(),
            layer.id()
        )));
        Ok(1)
    }
}

/// Resolves a user-supplied layer name (`first`, `1`, `FIRST`, …) to a layer.
fn layer_from(input: &str) -> Option<Layer> {
    let input = input.to_ascii_lowercase();
    Layer::ALL
        .into_iter()
        .find(|layer| layer.id() == input || layer.number().to_string() == input)
}

/// Returns the accepted layer names, for use in error messages.
fn layer_names() -> String {
    Layer::ALL
        .iter()
        .map(|layer| layer.id())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Suggests layer names for the `<layer>` argument.
///
/// **Disabled.** Attaching this with `CommandNode::suggest` makes the server panic on the
/// first tab-completion request: Pumpkin's host calls `block_in_place` from the
/// `Simulation` thread, which has no Tokio reactor, and panics before the plugin is
/// reached. The handler is kept, and gated behind `cfg(any())` — which is never true — so
/// it still compiles as part of `cargo check`, and enabling it is a matter of restoring
/// the two `.suggest(LayerSuggestions)` calls in [`register`] and dropping the gates.
///
/// It cannot be exercised until the host is fixed. See the notes in `AGENTS.md`.
#[cfg(any())]
#[derive(Debug, Default, Clone, Copy)]
struct LayerSuggestions;

#[cfg(any())]
impl CommandSuggestionHandler for LayerSuggestions {
    fn suggest(
        &self,
        _sender: CommandSender,
        _server: Server,
        request: SuggestionRequest,
    ) -> CommandSuggestions {
        let prefix = request.remaining.to_ascii_lowercase();

        let values = Layer::ALL
            .into_iter()
            .flat_map(|layer| [layer.id().to_string(), layer.number().to_string()])
            .filter(|candidate| candidate.starts_with(&prefix))
            .map(|value| CommandSuggestion {
                value,
                tooltip: None,
            })
            .collect();

        CommandSuggestions {
            start: request.start,
            length: u32::try_from(request.remaining.len()).unwrap_or(0),
            values,
        }
    }
}
