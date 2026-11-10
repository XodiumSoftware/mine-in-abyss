# TODO — Transition Logic Edge Cases

Findings from an audit of `src/events/layer_leave.rs`, checked against the
`pumpkin-plugin-api` WIT definitions (`PlayerMoveEventData`, `Player::teleport`).

## Root causes

- [x] **R1 — Global Y constants, per-layer ranges.** _(resolved by design)_ Every
      layer is its own full-height world, so the thresholds are intentionally shared:
      `Section::VOID_Y` / `Section::MAX_Y`, with `entry_y()` / `exit_y()` deriving the
      spawn heights. The misleading per-layer Y ranges were removed from the layer docs.
- [ ] **R2 — Handler is not one-shot.** `PlayerMoveEvent` fires every tick while a
      player stays in the trigger band, so `down()` / `up()` / `kill()` /
      `curse::apply` can run many times for a single intended transition.
- [ ] **R3 — Stale event after teleport.** The handler teleports from inside the
      move handler and returns the event uncancelled, so the server can process the
      original `to_position` against the new world.

## Edge cases

| Done | #   | Scenario                                               | Behavior today                                                        | Severity | Cause |
| ---- | --- | ------------------------------------------------------ | --------------------------------------------------------------------- | -------- | ----- |
| [x]  | 1   | Player below `VOID_Y` (spawn, `/tp`, portal)           | Latch: one teleport per transition                                    | Low      | —     |
| [x]  | 2   | Player in `Seventh` void band                          | Nothing happens; vanilla void damage kills them                       | Low      | —     |
| [x]  | 3   | Falling out of `Seventh` above Y `-128`                | Grey message: there is nothing below the Abyss                        | Low      | —     |
| [ ]  | 4   | Ascending into `First`                                 | Lands at `-127` — below the terrain, but the intended entry height    | Low      | —     |
| [ ]  | 5   | Descending into any layer                              | Lands at `318` — above the terrain, then falls in                     | Low      | —     |
| [ ]  | 6   | Any transition                                         | X/Z preserved but no safe-position fallback; can land in terrain      | Medium   | —     |
| [x]  | 7   | Re-entering the layer you just left                    | Latch stops the ping-pong loop                                        | Low      | —     |
| [ ]  | 8   | Curse applied on a re-entry loop                       | Latch prevents the loop, so strain cannot escalate                    | Low      | —     |
| [x]  | 9   | Another `PlayerMoveEvent` queued in the same tick      | Successful transition cancels the event, so the stale move is dropped | Low      | —     |
| [x]  | 10  | `get_world_by_name` returns `None`                     | `warn!` names the missing world; the teleport is skipped              | Low      | —     |
| [x]  | 11  | Layer disabled (`enabled() == false`, all seven today) | `enabled()` removed — every layer is live                             | Low      | —     |
| [ ]  | 12  | Player in a non-Abyss world                            | Correctly ignored                                                     | —        | —     |
| [ ]  | 13  | Manual `/tp` above `MAX_Y`                             | Treated as an ascent and cursed                                       | Medium   | —     |
| [x]  | 14  | `up()` from `First`                                    | Grey message: there is nothing above the Abyss                        | Low      | —     |
| [x]  | 15  | Player dies from strain 6 inside the move handler      | Dead players are skipped, and the latch releases after the move       | Low      | —     |
| [x]  | 16  | Creative / spectator player                            | `layer_leave` skips both (no teleport, no curse)                      | Low      | —     |
| [ ]  | 17  | `Seventh` boundaries                                   | No layer-specific boundaries — all layers share the vanilla limits    | Low      | —     |

## Fixes, in priority order

- [x] **F1 — Per-layer Y bounds.** _(resolved by design)_ The thresholds live on
      `Section` as `VOID_Y` / `MAX_Y`, with `entry_y()` and `exit_y()` deriving the
      spawn heights. Every layer is a full-height world, so they are intentionally
      shared; the misleading per-layer Y ranges were removed from the layer docs.
- [x] **F2 — One-shot latch.** _(done)_ A thread-local set of player ids
      (`utils::begin_transition` / `end_transition`) makes each transition fire once.
      Not persistent data: the latch only describes an in-flight transition, and a
      stale entry must never survive a restart. Addresses #1, #7, #8, #15. _(R2)_
- [x] **F3 — Cancel the move event.** _(done)_ `up`/`down` report whether the teleport
      happened, and a successful transition returns the event with `cancelled: true`, so
      the stale `to_position` is not applied. Addresses #9. _(R3)_
- [x] **F4 — Log missing worlds.** _(done)_ `world_for` in `layer_leave.rs` warns when
      the destination world is unavailable instead of returning silently. Addresses #10.
- [x] **F5 — `enabled()` semantics.** _(removed)_ `Layer::enabled()` was a display-only
      gate that did nothing, so it is gone: every layer is live and every layer world
      shows its entry title. Resolves #11.
- [x] **F6 — Skip dead and creative/spectator players.** _(done)_
      `utils::ignores_transitions` covers `GameMode::Creative`, `GameMode::Spectator`
      and dead players (`Player::as_entity().as_living().is_dead()`). Resolves #15, #16.
- [x] **F7 — Feedback on terminal transitions.** _(done)_ `tell` sends a grey system
      message when leaving `First` upward or `Seventh` downward. Addresses #14 and #3.

## Notes

- #12 needs no action; it is listed to record that the guard works.
- `Section::curse()` derives severity from the enum order; `entry_y()` / `exit_y()`
  now do the same for heights, so there is no second table to keep in sync.
