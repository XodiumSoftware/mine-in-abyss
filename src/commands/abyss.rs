//! The `/abyss` command.
//!
//! Moves a player into any layer of the Abyss directly, without ascending through the Abyss in
//! between, so the Curse of the Abyss is never inflicted. `item` gives the sender one custom
//! item, so the item path can be exercised before anything else produces one.
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
//! - `<item>` — a custom item's display name, case-insensitive, e.g. `beniguma fur`
//!
//! Tab-completion is implemented but disabled, for the same reason for both arguments: the
//! Pumpkin host panics on any suggestion request (see [`LayerSuggestions`]).
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

use crate::commands::command::Command as RegisterCommand;
use crate::entities::villagers::catalogue as traders;
use crate::items::catalogue;
use crate::layer::Layer;

/// The `/abyss` command, registration and executor.
///
/// The struct is empty: it exists so the command can implement [`RegisterCommand`] and
/// [`CommandHandler`], keeping both next to the tree they describe.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Abyss;

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

        let item = CommandNode::argument("item", &ArgumentType::String(StringType::Greedy));

        let entity = CommandNode::argument("entity", &ArgumentType::String(StringType::Greedy));

        let command = Command::new(&names, Self::description())
            .then(layer_with_player.then(player).execute(Abyss))
            .then(layer_leaf.execute(Abyss))
            .then(
                CommandNode::literal("item")
                    .then(item.execute(Abyss))
                    .execute(Abyss),
            )
            .then(
                CommandNode::literal("spawn")
                    .then(entity.execute(Abyss))
                    .execute(Abyss),
            )
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
        if let Arg::Simple(item) = args.get_value("item") {
            return Ok(give(&sender, &item));
        }

        if let Arg::Simple(entity) = args.get_value("entity") {
            return Ok(spawn(&sender, &entity));
        }

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

/// Gives the sender one item named `name`.
///
/// Only a player can be given one: the console has no inventory, and the argument names an
/// item rather than a recipient, so there is nothing to hand it to.
///
/// The first empty slot is filled rather than a fixed one, so repeated use does not overwrite
/// what was given before. A full inventory is reported rather than silently dropping the item.
fn give(sender: &CommandSender, name: &str) -> i32 {
    let Some(player) = sender.as_player() else {
        sender.send_error(TextComponent::text("Only a player can be given an item."));
        return 0;
    };

    let Some(stack) = catalogue::stack(name) else {
        sender.send_error(TextComponent::text(&format!(
            "Unknown item '{name}'. Expected one of: {}.",
            catalogue::NAMES.join(", "),
        )));
        return 0;
    };

    let inventory = player.get_inventory().as_inventory();
    let empty = (0..inventory.get_size()).find(|slot| inventory.get_item(*slot).is_none());

    let Some(slot) = empty else {
        sender.send_error(TextComponent::text("Your inventory is full."));
        return 0;
    };

    inventory.set_item(slot, Some(stack));

    sender.send_message(TextComponent::text(&format!("Gave one {name}.")));
    1
}

/// Spawns a mob or trader named `name` in front of the sender.
///
/// Mobs come from [`mob_kinds`](crate::mob_kinds), which already knows every kind, and
/// traders from their own table, since they have no catalogue. A mob kind's `LAYERS` is not
/// consulted: this places one wherever the sender stands, deliberately, the way a spawn egg
/// or `/summon` would — the layer list only gates the natural spawn rule.
///
/// The entity is placed two blocks in front of the sender's facing, so it does not appear
/// inside them.
fn spawn(sender: &CommandSender, name: &str) -> i32 {
    let Some(player) = sender.as_player() else {
        sender.send_error(TextComponent::text("Only a player can spawn an entity."));
        return 0;
    };

    let world = player.as_entity().get_world();

    let (x, y, z) = player.get_position();
    let position = (x + 2.0, y, z + 2.0);

    if let Some(kind) = crate::mob_kinds().into_iter().find(|k| k.is_named(name)) {
        (kind.spawn)(&world, position);
        sender.send_message(TextComponent::text(&format!("Spawned {}.", kind.name)));
        return 1;
    }

    if let Some(constructor) = traders::spawn(name) {
        constructor(&world, position);
        sender.send_message(TextComponent::text(&format!("Spawned {name}.")));
        return 1;
    }

    sender.send_error(TextComponent::text(&format!(
        "Unknown entity '{name}'. Expected a mob ({}) or a trader ({}).",
        mob_names().join(", "),
        traders::NAMES.join(", "),
    )));
    0
}

/// Every mob name `/abyss spawn` accepts, for the unknown-entity message.
fn mob_names() -> Vec<&'static str> {
    crate::mob_kinds()
        .into_iter()
        .map(|kind| kind.name)
        .collect()
}

/// Suggests layer names for the `<layer>` argument.
///
/// **Disabled.** Attaching this with `CommandNode::suggest` makes the server panic on the
/// first tab-completion request: Pumpkin's host calls `block_in_place` from the
/// `Simulation` thread, which has no Tokio reactor, and panics before the plugin is
/// reached. The handler is kept, gated behind `cfg(any())` — which is never true — so it
/// still compiles as part of `cargo check`; enabling it means restoring the
/// `.suggest(LayerSuggestions)` calls in [`Abyss`]'s `register` and dropping the gates.
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
