---
name: add-event
description: Adds a new event handler to mine-in-abyss by creating a module under src/events, implementing Event beside its EventHandler, and adding it to the MineInAbyss::events array.
---

# Add an Event Handler

Handlers live in `src/events/<name>.rs` and implement
`pumpkin_plugin_api::events::EventHandler<E>`. Each one is registered once in
`MineInAbyss::register_events` (`src/lib.rs`).

**Discover, don't assume.** Read `src/lib.rs` (the `mod events` block and the `MineInAbyss::events`
array) and `src/events/layer_teleport.rs` (the smallest complete handler) before writing
anything. Event names and data shapes come from the crate's WIT, not from memory.

An earlier version of this skill said to register in `MineInAbyss::register_events` with a
`register_event_handler::<Marker, _>(handler, EventPriority::Normal, false)` call. **Neither
exists.** Handlers implement the `Event` trait in `src/events/event.rs`, which owns the
priority, the blocking flag and the `register` body; `MineInAbyss::events` calls each
`register` from one array.

## Step 0 — Find the event

1. List the available events:

    ```bash
    ls ~/.cargo/registry/src/*/pumpkin-plugin-api-*/src/events/*/
    ```

2. Read the module for the event. It gives three things:
    - the marker type, e.g. `PlayerRespawnEvent`
    - its path, e.g. `pumpkin_plugin_api::events::player::player_respawn::PlayerRespawnEvent`
    - its data type, e.g. `pumpkin_plugin_api::events_wit::PlayerRespawnEventData`

3. Read the matching `record …-event-data` in `wit/event.wit` for the fields, and check
   whether it has a `cancelled` field — only then can the handler suppress the action.

## Implementation

1. Create `src/events/<name>.rs` with `//!` module docs saying what the handler does and why,
   then two impls: the `EventHandler` itself, and `Event` beside it so the event type, the
   priority and the blocking flag stay in the file that defines the handler.

    ```rust
    use pumpkin_plugin_api::Context;
    use pumpkin_plugin_api::Server;
    use pumpkin_plugin_api::events::EventHandler;
    use pumpkin_plugin_api::events::<group>::<module>::Marker;
    use pumpkin_plugin_api::events_wit::Data;

    use crate::events::event::Event;

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Name;

    impl Event for Name {
        fn register(context: &Context) {
            context
                .register_event_handler::<Marker, _>(Name, Self::PRIORITY, Self::BLOCKING)
                .unwrap_or_else(|_| Self::fail("Name"));
        }
    }

    impl EventHandler<Marker> for Name {
        fn handle(&self, _server: Server, event: Data) -> Data {
            event
        }
    }
    ```

2. Declare it in the `mod events` block in `src/lib.rs`.
3. Add one entry to the array in `MineInAbyss::events`, and bump its length:

    ```rust
    let events: [fn(&Context); 6] = [
        events::<name>::Name::register,
    ];
    ```

   The array carries its own `len()`, so the logged count cannot drift from what was
   registered.

## Defaults on `Event`

- `PRIORITY` — `EventPriority::Normal`. Override only when the handler must run before or
  after another handler of the same event.
- `BLOCKING` — `false`. Every current handler uses `false`; the handlers here touch the world,
  which is not thread-safe. Override only for work that never touches it.
- `fail(handler)` — the shared panic tail, so a refused registration fails loudly during
  `on_load` rather than silently doing nothing. Use it instead of an `expect(..)` per file.

## Constraints

- **Return `event`**, modified or not. The handler receives the data by value; dropping the
  return value silently discards any change.
- **`cancelled` is opt-in.** Set `Data { cancelled: true, ..event }` only when the action
  should be suppressed — see `layer_leave.rs`.
- **One handler for all layers.** Look the layer up with `crate::layer_of(..)` and match on it,
  rather than registering a handler per layer — see `layer_leave.rs`.
- Skip players who should not be affected, through `crate::utils::ignores_transitions(..)`
  (creative, spectator and dead), rather than re-implementing the check per handler.
- Per-layer **behaviour** goes in a handler; per-layer **data** belongs on `Layer` in
  `src/layer.rs`.
- Teleporting or changing the player inside a handler does not update the event data, so the
  rest of the handler still sees the original positions.
- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- `layer_join.rs` and `layer_leave.rs` share invariants (the transition latch, the teleport
  record). Read both before changing either.

## Verification

Run `just validate` and `just doc`. The release build is required: debug builds fail to load in
the Pumpkin runtime.

Then add the handler to the Events table in `AGENTS.md`.

Event behaviour can only be confirmed in a running Pumpkin server, so state in the summary
which assumptions are untested.

After finishing, summarize the files changed and ask the user whether to commit.
