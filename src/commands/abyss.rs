//! The `/abyss` command.
//!
//! Moves a player into any layer of the Abyss directly, without ascending through the
//! Abyss in between, so the Curse of the Abyss is never inflicted.
//!
//! It also carries `/abyss item`, which puts a custom item in the sender's hand. That
//! subcommand exists to exercise the [`ItemBuilder`](crate::items::item_builder::ItemBuilder)
//! path end to end — create a stack, stamp it, read the marker back — until a real source of
//! items (loot tables or crafting) exists.
//!
//! # Usage
//!
//! ```text
//! /abyss <layer> [player]
//! /abyss item <item>
//! ```
//!
//! - `<layer>` — `orth`, `first` … `seventh`, or `1` … `8`, case-insensitive
//! - `[player]` — who to move; only used when the command comes from **console**, since a
//!   player sender is always moved themselves
//! - `<item>` — `relic`, case-insensitive; gives one Abyss Relic to the sender
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
use pumpkin_plugin_api::wit::pumpkin::plugin::inventory::Hand;
use pumpkin_plugin_api::wit::pumpkin::plugin::permission::{
    Permission, PermissionDefault, PermissionLevel,
};
use pumpkin_plugin_api::{Context, Server};

use crate::commands::command::Command as RegisterCommand;
use crate::items::abyss_relic::AbyssRelic;
use crate::items::item_builder::{ItemBuilder, id_of};
use crate::layer::Layer;

/// The `/abyss` command, registration and executor.
///
/// The struct is empty: it exists so the command can implement [`RegisterCommand`] and
/// [`CommandHandler`], keeping both next to the tree they describe.
#[derive(Debug, Default, Clone, Copy)]
pub struct Abyss;

impl RegisterCommand for Abyss {
    fn names() -> &'static [&'static str] {
        &["abyss", "mia"]
    }

    fn description() -> &'static str {
        "Move to a layer of the Abyss without the curse"
    }

    fn register(context: &Context) {
        let names: Vec<String> = Self::names().iter().map(ToString::to_string).collect();

        let layer_leaf =
            CommandNode::argument("layer", &ArgumentType::String(StringType::SingleWord));
        let layer_with_player =
            CommandNode::argument("layer", &ArgumentType::String(StringType::SingleWord));
        let player = CommandNode::argument("player", &ArgumentType::String(StringType::SingleWord));

        let item = CommandNode::literal("item");
        let item_name =
            CommandNode::argument("item", &ArgumentType::String(StringType::SingleWord));

        let command = Command::new(&names, Self::description())
            .then(item.then(item_name).execute(AbyssItem))
            .then(layer_with_player.then(player).execute(Abyss))
            .then(layer_leaf.execute(Abyss))
            .execute(Abyss);

        context.register_command(command, &Self::permission_node());

        context
            .register_permission(&Permission {
                node: Self::permission_node(),
                description: Self::description().to_string(),
                default: PermissionDefault::Op(PermissionLevel::Three),
                children: vec![],
            })
            .unwrap_or_else(|_| Self::fail("Abyss"));
    }
}

impl CommandHandler for Abyss {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let Arg::Simple(layer) = args.get_value("layer") else {
            return Err(CommandError::InvalidConsumption(None));
        };

        let Some(layer) = Layer::from_id(&layer) else {
            sender.send_error(TextComponent::text(&format!(
                "Unknown layer '{layer}'. Expected one of: {}.",
                Layer::id_list()
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

/// The `/abyss item <item>` executor.
///
/// Creates the named custom item, puts it in the sender's hand, and reads the marker straight
/// back off the stored stack. That round trip is the point: it proves the stack survives
/// being written into an inventory slot and still reports its own kind.
#[derive(Debug, Default, Clone, Copy)]
pub struct AbyssItem;

impl CommandHandler for AbyssItem {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let Arg::Simple(name) = args.get_value("item") else {
            return Err(CommandError::InvalidConsumption(None));
        };

        let Some(player) = sender.as_player() else {
            sender.send_error(TextComponent::text("Console must run this as a player."));
            return Ok(0);
        };

        if !name.eq_ignore_ascii_case("relic") {
            sender.send_error(TextComponent::text(&format!(
                "Unknown item '{name}'. Expected one of: relic."
            )));
            return Ok(0);
        }

        let stack = AbyssRelic::create(1);
        player
            .get_inventory()
            .set_item_in_hand(Hand::Right, Some(stack));

        let stored = player.get_inventory().get_item_in_hand(Hand::Right);
        let identity = stored.as_ref().and_then(id_of);

        if identity == Some(AbyssRelic::id()) {
            sender.send_message(TextComponent::text(
                "Gave one Abyss Relic; marker reads back.",
            ));
            return Ok(1);
        }

        sender.send_error(TextComponent::text(
            "Created an Abyss Relic, but its marker did not survive the inventory slot.",
        ));
        Ok(0)
    }
}

/// Suggests layer names for the `<layer>` argument.
///
/// **Disabled.** Attaching this with `CommandNode::suggest` makes the server panic on the
/// first tab-completion request: Pumpkin's host calls `block_in_place` from the
/// `Simulation` thread, which has no Tokio reactor, and panics before the plugin is
/// reached. The handler is kept, and gated behind `cfg(any())` — which is never true — so
/// it still compiles as part of `cargo check`, and enabling it is a matter of restoring
/// the two `.suggest(LayerSuggestions)` calls in [`Abyss`]'s `register` and dropping the
/// gates.
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
