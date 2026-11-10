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
//! - `[player]` — who to move; defaults to the sender, so console must name a player
//!
//! # Permissions
//!
//! | Node                          | Default                    |
//! | ----------------------------- | -------------------------- |
//! | `mine_in_abyss:command.abyss` | granted to ops at level 3+ |

use pumpkin_plugin_api::command::{
    Arg, ArgumentType, Command, CommandError, CommandNode, CommandSender, ConsumedArgs, StringType,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::permission::{
    Permission, PermissionDefault, PermissionLevel,
};
use pumpkin_plugin_api::{Context, Server};

use crate::PLUGIN_ID;
use crate::section::Section;

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

        let Some(section) = section_from(&layer) else {
            sender.send_error(TextComponent::text(&format!(
                "Unknown layer '{layer}'. Expected one of: {}.",
                section_names()
            )));
            return Ok(0);
        };

        let target = match args.get_value("player") {
            Arg::Simple(name) => server.get_player_by_name(&name),
            _ => sender.as_player(),
        };

        let Some(player) = target else {
            sender.send_error(TextComponent::text(
                "That player is not online, or you must name one when running this from console.",
            ));
            return Ok(0);
        };

        let Some(world) = server.get_world_by_name(&section.world()) else {
            sender.send_error(TextComponent::text(&format!(
                "The world for {} is not available.",
                section.id()
            )));
            return Ok(0);
        };

        let (x, _, z) = player.get_position();
        player.teleport((x, Section::entry_y(), z), None, None, world);

        sender.send_message(TextComponent::text(&format!(
            "Moved {} to {}.",
            player.get_name(),
            section.id()
        )));
        Ok(1)
    }
}

/// Resolves a user-supplied layer name (`first`, `1`, `FIRST`, …) to a section.
fn section_from(input: &str) -> Option<Section> {
    let input = input.to_ascii_lowercase();
    Section::ALL
        .into_iter()
        .find(|section| section.id() == input || section.number().to_string() == input)
}

/// Returns the accepted layer names, for use in error messages.
fn section_names() -> String {
    Section::ALL
        .iter()
        .map(|section| section.id())
        .collect::<Vec<_>>()
        .join(", ")
}
