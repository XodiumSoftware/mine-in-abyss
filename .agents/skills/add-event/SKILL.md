---
name: add-event
description: Adds a new event handler to mine-in-abyss by creating a module under src/events and registering it in MineInAbyss::register_events.
---

# Add an Event Handler

Handlers live in `src/events/<name>.rs` and implement
`pumpkin_plugin_api::events::EventHandler<E>`. Each one is registered once in
`MineInAbyss::register_events` (`src/lib.rs`).

**Discover, don't assume.** Read `src/lib.rs` (the `mod events` block and
`register_events`) and `src/events/layer_respawn.rs` (the smallest complete handler) before
writing anything. Event names and data shapes come from the crate's WIT, not from memory.

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
   then the imports, then a zero-sized handler:

    ```rust
    use pumpkin_plugin_api::Server;
    use pumpkin_plugin_api::events::EventHandler;
    use pumpkin_plugin_api::events::<group>::<module>::Marker;
    use pumpkin_plugin_api::events_wit::Data;

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Name;

    impl EventHandler<Marker> for Name {
        fn handle(&self, _server: Server, event: Data) -> Data {
            event
        }
    }
    ```

2. Declare it in the `mod events` block in `src/lib.rs`.
3. Register it in `MineInAbyss::register_events`:

    ```rust
    context
        .register_event_handler::<Marker, _>(
            events::<name>::Name,
            EventPriority::Normal,
            false,
        )
        .expect("failed to register Marker handler");
    ```

## Constraints

- **Return `event`**, modified or not. The handler receives the data by value; dropping the
  return value silently discards any change.
- **`cancelled` is opt-in.** Set `Data { cancelled: true, ..event }` only when the action
  should be suppressed — see `layer_leave.rs`.
- **`blocking: false`** unless the handler must finish before others run. Every current
  handler uses `false`.
- **One handler for all layers.** Look the layer up with `crate::layer_for_world(...)` and
  match on it, rather than registering a handler per layer — see `layer_leave.rs`.
- Per-layer **behaviour** goes in a handler; per-layer **data** belongs on `Layer` in
  `src/layer.rs`.
- Teleporting or changing the player inside a handler does not update the event data, so the
  rest of the handler still sees the original positions.
- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- `layer_join.rs` and `layer_leave.rs` share invariants (the transition latch, the teleport
  record). Read both before changing either.

## Verification

```bash
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
cargo build --target wasm32-wasip2
cargo doc --no-deps --target wasm32-wasip2
```

Then add the handler to the Events table in `AGENTS.md`.

Event behaviour can only be confirmed in a running Pumpkin server, so state in the summary
which assumptions are untested.

After finishing, summarize the files changed and ask the user whether to commit.
