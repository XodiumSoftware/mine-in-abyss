---
name: add-util
description: Adds a shared helper to mine-in-abyss by creating a module under src/utils, re-exporting it from the utils block in lib.rs, and keeping any thread-local state beside the functions that maintain it.
---

# Add a Util

`src/utils/` holds the small helpers more than one module needs. It is organised **by what a
module owns**, not by type: a file is a category of helper, and anything with state owns that
state too.

**Discover, don't assume.** Read these before writing anything:

1. The `mod utils` block in `src/lib.rs` — the submodule list and the `pub use` re-exports.
2. `src/utils/general.rs` — the simplest module: pure functions, no state.
3. `src/utils/fall.rs` — the worked example of a stateful module.
4. `src/utils/teleport.rs` and `src/utils/transition.rs` — the other two stateful ones.

## The current layout

| Module | Holds | Re-exported? |
| ------ | ----- | ------------ |
| `general.rs` | `key`, `is_dead`, `exempt_gamemode` — per-player predicates | **No** — internal |
| `fall.rs` | the carried fall distance of a descent | Yes |
| `teleport.rs` | "this move is a teleport, not a climb" | Yes |
| `transition.rs` | the one-shot transition latch | Yes |
| `text.rs` | `tell`, `ordinal`, `rgb` | Yes |
| `world.rs` | `world_for`, `layer_of` | Yes |

`general.rs` is deliberately not re-exported: it is plumbing other utils modules call
(`fall.rs`, `teleport.rs` and `transition.rs` all use `general::key`), not an API a handler
should reach for directly. Follow that: re-export what callers of `utils` should use.

## Step 1 — Decide whether it belongs in `utils` at all

Put it in `utils` only if **two or more** modules need it. Otherwise it belongs in the file
that uses it, or — if it is per-layer data or behaviour — on `Layer` or in the relevant event
handler. A helper used once is indirection, not reuse.

## Step 2 — Write the module

`src/utils/<name>.rs`, opening with `//!` docs that say what the helper owns and why it
exists. Every `pub` item gets `///` docs, and pure functions that do not mutate get
`#[must_use]`.

```rust
//! Resolving a layer from a world, and a world from a layer.

use pumpkin_plugin_api::{Player, Server, World};
use tracing::warn;

use crate::Layer;

/// Returns the layer `player` is standing in, if any.
#[must_use]
pub fn layer_of(player: &Player) -> Option<Layer> {
    crate::layer_for_world(&player.get_world().get_name())
}
```

## Step 3 — State belongs in the same file

If the helper needs memory between events, the `thread_local` goes **in that module**, right
next to the functions that read and write it, so the state and its accessors cannot drift
apart. This is the pattern every stateful util follows:

```rust
thread_local! {
    /// What the record means, when it is written, when it is read, and when it is dropped.
    ///
    /// Say explicitly whether it is persistent across a restart and why.
    static TELEPORTED: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
}
```

Rules that hold for all of them:

- **Key on `general::key(player)`**, never the `Player` handle. A handle from one event is not
  guaranteed to compare equal to one from another; the UUID always is.
- **Use `thread_local!` + `RefCell`, not `static mut`.** Every existing record follows this, and
  `unsafe_code` is forbidden project-wide, so a `static mut` is not an option anyway.
- **Say whether it survives a restart.** An in-flight transition, a carried fall and a
  pending teleport are all *deliberately not persistent* — a stale entry could teleport,
  injure or lock out a player with no memory of the event. Only real player state (the curse
  strain) uses `PersistentDataHolder`; see the `add-persistent-data` skill.
- **Remove entries**, do not leak them. `consume_teleport` removes on read; `clear_fall` exists
  so a teleport or respawn drops a stale distance.

## Step 4 — Declare and re-export in `lib.rs`

Add `pub mod <name>;` to the `mod utils` block, and a `pub use` line for anything callers
should reach. Callers then write `use crate::utils::{carry_fall, layer_of};` and never need to
know which file a helper lives in; `utils::<module>::…` is for when the grouping is the point.

If the helper is internal to `utils`, declare the module but add **no** re-export — that is
how `general` stays private.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- Do not return a WIT resource (such as `World` or `Player`) and hold it in a `thread_local`;
  they are non-cloneable handles. Cache the **name or id** and re-resolve the handle, as
  `Worlds::READY` does.
- Utilities must not silently swallow a failure. `world_for` takes a `purpose: &str` and
  `warn!`s, so a caller can log *why* the missing world mattered.
- Keep thresholds as named `const`s (`TELEPORT_TOLERANCE`), not magic numbers in a comparison.

## Verification

Run `just validate` (lint + fmt-check + release build) and `just doc`. The release build is
required: debug builds fail to load in the Pumpkin runtime.

Then update `AGENTS.md` § Project Structure: the `src/utils/` tree lists every module with a
one-line description, so add the new file there. If the helper changes an invariant that a
handler depends on (as the latch and the carried fall do), say so in `ARCHITECTURE.md` too.

Helper behaviour can only be confirmed in a running Pumpkin server, so state in the summary
which assumptions are untested.

After finishing, summarize the files changed and ask the user whether to commit.
