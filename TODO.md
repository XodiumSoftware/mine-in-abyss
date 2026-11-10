# TODO — Transition Logic Edge Cases

Findings from an audit of `src/events/layer_leave.rs`, checked against the
`pumpkin-plugin-api` WIT definitions (`PlayerMoveEventData`, `Player::teleport`,
`PlayerChangedWorldEventData`).

All three root causes and all seven fixes are resolved. Nothing here is
compiler-verified beyond clippy/build/doc: **none of the transition work has been run
against a live Pumpkin server yet.** See [Verification](#verification).

## Root causes

- [x] **R1 — Global Y constants, per-layer ranges.** _(resolved by design)_ Every layer
      is its own full-height world, so the thresholds are intentionally shared:
      `Section::VOID_Y` / `Section::MAX_Y`. The misleading per-layer Y ranges were
      removed from the layer module docs.
- [x] **R2 — Handler is not one-shot.** `PlayerMoveEvent` fires every tick while a
      player stays in the trigger band, so `down()` / `up()` / `curse::apply` could run
      many times for one intended transition. _(fixed by F2)_
- [x] **R3 — Stale event after teleport.** The handler teleported from inside the move
      handler and returned the event uncancelled. _(fixed by F3)_

## Edge cases

`Cause` records which root cause produced the finding; all three are now closed.

| Done | #   | Scenario                                          | Behavior today                                                    | Cause |
| ---- | --- | ------------------------------------------------- | ----------------------------------------------------------------- | ----- |
| [x]  | 1   | Player below `VOID_Y` (spawn, `/tp`, portal)      | Latch holds until the world change: one teleport per transition   | R2    |
| [x]  | 2   | Player in `Seventh` void band                     | Nothing happens; vanilla void damage kills them                   | R2    |
| [x]  | 3   | Falling out of `Seventh` above Y `-128`           | Grey message: there is nothing below the Abyss                    | R1    |
| [x]  | 4   | Ascending into `First`                            | Lands at `-127` and climbs out — intended entry height, not a bug | R1    |
| [x]  | 5   | Descending into any layer                         | Lands at `318` and falls in — intended entry height, not a bug    | R1    |
| [x]  | 6   | Any transition                                    | X/Z preserved, no safe-position lookup. By design: see below      | R1    |
| [x]  | 7   | Re-entering the layer you just left               | Latch stops the ping-pong loop                                    | R2    |
| [x]  | 8   | Curse applied on a re-entry loop                  | No loop, so strain cannot escalate                                | R2    |
| [x]  | 9   | Another `PlayerMoveEvent` queued in the same tick | Successful transition cancels the event, dropping the stale move  | R3    |
| [x]  | 10  | `get_world_by_name` returns `None`                | `warn!` names the missing world; the teleport is skipped          | R1    |
| [x]  | 11  | Layer disabled (`enabled() == false`)             | `enabled()` removed — every layer is live                         | —     |
| [x]  | 12  | Player in a non-Abyss world                       | Correctly ignored. Working as intended, no action needed          | —     |
| [ ]  | 13  | Manual `/tp` above `MAX_Y`                        | Treated as an ascent and cursed. Not seen in testing: see below   | —     |
| [x]  | 14  | `up()` from `First`                               | Grey message: there is nothing above the Abyss                    | R1    |
| [x]  | 15  | Player dies from strain 6 inside the move handler | Dead players skipped; successful transitions stay latched         | R2    |
| [x]  | 16  | Creative / spectator player                       | `ignores_transitions` skips both, plus dead players               | —     |
| [x]  | 17  | `Seventh` boundaries                              | No per-layer boundaries — all layers share the vanilla limits     | R1    |

## Fixes

- [x] **F1 — Shared Y thresholds.** _(resolved by design)_ `Section::VOID_Y` /
      `Section::MAX_Y` hold the vanilla Overworld limits; `entry_y()` / `exit_y()`
      return `MAX_Y - 2.0` and `VOID_Y + 1.0`. These are the **same for every section** —
      they do not vary by depth the way `Section::curse()` does.
- [x] **F2 — One-shot latch.** A thread-local set of player ids
      (`utils::begin_transition` / `end_transition`) makes each transition fire once.
      **Held until `PlayerChangedWorldEvent`** — `LayerLeave` takes it, `LayerJoin`
      releases it — so the moves after a teleport cannot re-trigger it. A transition
      that cannot finish releases it itself. Not persistent: a stale entry must never
      survive a restart.
- [x] **F3 — Cancel the move event.** `up`/`down` report whether the teleport happened,
      and a successful transition returns the event with `cancelled: true`.
- [x] **F4 — Log missing worlds.** `world_for` warns when the destination world is
      unavailable instead of returning silently.
- [x] **F5 — `enabled()` removed.** It was a display-only gate that returned false for
      every layer, so no entry title had ever been shown. Every layer is now live.
- [x] **F6 — Skip dead and creative/spectator players.**
      `utils::ignores_transitions` covers `GameMode::Creative`, `GameMode::Spectator`
      and `Player::as_entity().as_living().is_dead()`, failing open when the entity
      cannot be inspected.
- [x] **F7 — Feedback on terminal transitions.** `tell` sends a grey system message when
      leaving `First` upward or `Seventh` downward. No repeat-spam was observed in testing.

## Open questions

- **#13 — manual `/tp` above `MAX_Y`.** Still indistinguishable from a real ascent, so an
  operator teleporting overhead is cursed. Not observed in testing — ops are usually in
  creative/spectator, which `ignores_transitions` already exempts. Decide only if it
  bites; the fix would be guarding on `PlayerTeleportEvent`.

Everything else is closed. `#6` is intentional (per `AGENTS.md`, each layer is a
hand-built map, so a guaranteed arrival point belongs in map data, never a hardcoded
coordinate), and `#12` records that the non-Abyss guard works.

## Verification

**Confirmed working in a live Pumpkin server.** The transition fixes (F2–F7) are
validated against a running server, not just compiled: the latch releases on the world
change, cancelled moves are honoured, dead-player and gamemode exemptions hold, and the
terminal messages behave.

Compiler checks that back each commit:

```bash
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
cargo build --target wasm32-wasip2
cargo doc --no-deps --target wasm32-wasip2
```

## Notes

- `Section::curse()` derives severity (0–6) from the enum order. `entry_y()` and
  `exit_y()` are **associated functions with no receiver** because they do not vary by
  section: every layer is a full-height world sharing the same two limits. Passing a
  `Section` there would imply a per-layer height that does not exist.
- The latch assumes single-threaded plugin execution, matching `Worlds` and the WASM
  target. If Pumpkin ever dispatches events across threads this stops working.
