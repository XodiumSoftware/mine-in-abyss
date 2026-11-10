---
name: add-persistent-data
description: Adds per-player or per-world state to mine-in-abyss, choosing between persistent NBT and thread-local state.
---

# Add Persistent Data

The plugin keeps two kinds of state, and picking the wrong one fails silently. **Answer this
first: must the value survive a server restart?**

| Answer  | Where it goes                               | Examples                               |
| ------- | ------------------------------------------- | -------------------------------------- |
| **Yes** | Player/world NBT via `PersistentDataHolder` | `curse` strain (`src/events/curse.rs`) |
| **No**  | `thread_local!` in `src/utils.rs`           | `TRANSITING`, `TELEPORTED`             |

Getting this backwards is the whole reason this skill exists:

- Persistent state that should be transient **locks players out after a crash** — a latch left
  set because the process died mid-transition would never clear.
- Transient state that should persist **silently forgets** — curse strain must follow a player
  through a relog, or the penalty resets for free.

## Persistent (survives restart)

`src/events/curse.rs` is the worked example.

1. Pick a **namespaced** key — never a bare string, or you collide with other plugins:

    ```rust
    const STRAIN_KEY: &str = "curse";
    player.set_byte(PLUGIN_ID, STRAIN_KEY, value);
    ```

2. Use the typed helpers on `PersistentDataHolder` (`get_string`, `get_int`, `get_byte`,
   `get_float`, …). The WIT tree also takes raw `NbtTag`s if you need a compound.

3. **Mind the integer width.** NBT `Byte` is `i8`, so a `u8` domain needs conversion in both
   directions — `curse.rs` does `u8::try_from(..)` on read and `i8::try_from(..)` on write.

4. **Clear by removing, not by writing zero**, so absent and "zero" are the same state:

    ```rust
    if strain == 0 {
        player.remove_custom_data(PLUGIN_ID, STRAIN_KEY);
    }
    ```

5. Read defensively — a missing or wrong-typed value should mean "default", not a panic.

## Transient (restart-forgotten)

`src/utils.rs` is the worked example.

1. Put the `thread_local!` in `src/utils.rs` beside the existing ones, with a doc comment
   saying **why it must not persist**.

2. Key by player id, through the crate-private `key(player)` helper already in `utils.rs`, rather
   than by a WIT handle — handles are not stable across calls and are not `Hash`. `key` is
   module-private: either reuse it from inside `utils.rs` or make it `pub(crate)` first.

3. Prefer **consume-on-read** for one-shot markers. `consume_teleport` removes the record as it
   reads it, so it cannot suppress a later, genuine event.

4. Note the threading assumption: Pumpkin dispatches plugin calls on one thread, matching
   `Worlds` and the WASM target. State breaks if that ever changes — say so in the doc comment.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- State that gates player movement belongs with the transition latch — read
  `src/events/layer_join.rs` and `src/events/layer_leave.rs` before adding another one, since
  a latch that is never released is worse than no latch.
- A new persistent key is a **save-format** decision: changing its name or type later silently
  discards existing player data. Document it in the module docs.

## Verification

Run `just lint`, `just build` and `just doc` — the recipes are release-targeted and warnings
are errors. (`just validate` also runs the format check, as CI does.)

State behaviour needs a live server: restart to confirm persistence, and relog to confirm it
survives a disconnect. Say in the summary which of the two you checked.

After finishing, summarize the files changed and ask the user whether to commit.
