---
name: add-persistent-data
description: Adds per-player or per-world state to mine-in-abyss, choosing between persistent NBT and thread-local state.
---

# Add Persistent Data

The plugin keeps two kinds of state, and picking the wrong one fails silently. **Answer this
first: must the value survive a server restart?**

| Answer  | Where it goes                                        | Examples                                        |
| ------- | ---------------------------------------------------- | ----------------------------------------------- |
| **Yes** | Player NBT via `PersistentDataHolder`                | `curse` strain (`src/events/curse.rs`)          |
| **No**  | A `thread_local!` inside a module under `src/utils/` | `TRANSITING`, `TELEPORTED`, `FALL_CARRY`        |

Getting this backwards is the whole reason this skill exists:

- Persistent state that should be transient **locks players out after a crash** — a latch left
  set because the process died mid-transition would never clear.
- Transient state that should persist **silently forgets** — curse strain must follow a player
  through a relog, or the penalty resets for free.

**Discover, don't assume.** Read `src/events/curse.rs` and `src/utils/fall.rs` before writing
anything. An earlier version of this skill pointed at a single `src/utils.rs`, a module-private
`key(player)` and a threading assumption in `Worlds`; **none of that is true**. `utils` is a
folder of per-category modules, `key` is a `pub fn` in `utils/general.rs`, and `Worlds` caches
names in a `thread_local`, not player handles.

## Persistent (survives restart)

`src/events/curse.rs` is the worked example.

1. Pick a **namespaced** key — always under `PLUGIN_ID`, never your own namespace:

    ```rust
    const STRAIN_KEY: &str = "curse";
    player.set_byte(PLUGIN_ID, STRAIN_KEY, value);
    ```

    The marker helpers for mobs and items use the same mechanism under the keys `"mob"` and
    `"item"`, so those keys are taken.

2. Use the typed helpers on `PersistentDataHolder` — `set_string`/`get_string`, `set_int`/
   `get_int`, `set_long`/`get_long`, `set_short`, `set_byte`, `set_bool`, `set_float`,
   `set_double`, and the `*_array` variants. Every one takes `(namespace, key, ..)`.

3. **`set_custom_data` takes an `NbtTree`, which cannot be built from Rust** — it is an opaque
   handle with no constructor. The typed helpers exist precisely so you never need it; do not
   design around building a compound.

4. **Mind the integer width.** NBT `Byte` is `i8`, so a `u8` domain needs conversion in both
   directions — `curse.rs` does `u8::try_from(..)` on read and `i8::try_from(..)` on write, and
   a value that does not fit is silently not written. Prefer `set_long`/`get_long` for a wide
   counter rather than squeezing it into a byte.

5. **Clear by removing, not by writing zero**, so absent and "zero" are the same state:

    ```rust
    if strain == 0 {
        player.remove_custom_data(PLUGIN_ID, STRAIN_KEY);
    }
    ```

6. Read defensively — a missing or wrong-typed value should mean "default", not a panic.

## Transient (restart-forgotten)

`src/utils/fall.rs` is the worked example of a stateful util; `src/utils/teleport.rs` and
`src/utils/transition.rs` are the other two.

1. Create a module under `src/utils/`, declare it in the `mod utils` block in `lib.rs`, and put
   the `thread_local!` **in that module** beside the functions that read and write it — the state
   and its accessors must not be able to drift apart. Follow the `add-util` skill for the full
   shape.

2. Key on `general::key(player)`, never the `Player` handle: a handle from one event does not
   compare equal to one from another, but the UUID does.

3. Prefer **consume-on-read** for one-shot markers. `consume_teleport` removes the record as it
   reads it, so it cannot suppress a later, genuine event.

4. Say in the `thread_local!` doc comment **why it must not persist**. Every existing record does:
   a stale latch, fall or teleport could teleport, injure or lock out a player with no memory of
   the event.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- State that gates player movement belongs with the transition latch — read
  `src/events/layer_join.rs` and `src/events/layer_leave.rs` before adding another one, since
  a latch that is never released is worse than no latch.
- A new persistent key is a **save-format** decision: changing its name or type later silently
  discards existing player data. Document it in the module docs.
- Do not store a WIT resource (`World`, `Player`, `ItemStack`) in a `thread_local` — they are
  non-cloneable handles. Cache the name or id and re-resolve, as `Worlds::READY` does.

## Verification

Run `just validate` (lint + fmt-check + release build) and `just doc`. The release build is
required: debug builds fail to load in the Pumpkin runtime.

State behaviour needs a live server: restart to confirm persistence, and relog to confirm it
survives a disconnect. Say in the summary which of the two you checked, and state clearly when
only the compile-time checks have run.

After finishing, summarize the files changed and ask the user whether to commit.
