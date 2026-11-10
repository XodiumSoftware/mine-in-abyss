# TODO — Transition Logic Edge Cases

Findings from an audit of `src/events/layer_leave.rs`, checked against the
`pumpkin-plugin-api` WIT definitions (`PlayerMoveEventData`, `Player::teleport`).

## Root causes

- [ ] **R1 — Global Y constants, per-layer ranges.** `VOID_Y` and `MAX_Y` are
      hardcoded, but every layer documents a different Y range. Transitions place
      the player at `-127` or `318` regardless of the destination layer.
- [ ] **R2 — Handler is not one-shot.** `PlayerMoveEvent` fires every tick while a
      player stays in the trigger band, so `down()` / `up()` / `kill()` /
      `curse::apply` can run many times for a single intended transition.
- [ ] **R3 — Stale event after teleport.** The handler teleports from inside the
      move handler and returns the event uncancelled, so the server can process the
      original `to_position` against the new world.

## Edge cases

| Done | #   | Scenario                                               | Behavior today                                                        | Severity | Cause  |
| ---- | --- | ------------------------------------------------------ | --------------------------------------------------------------------- | -------- | ------ |
| [ ]  | 1   | Player below `VOID_Y` (spawn, `/tp`, portal)           | Every move re-arms `down()`; teleports repeatedly to an arbitrary X/Z | High     | R2     |
| [ ]  | 2   | Player in `Seventh` void band                          | `player.kill()` fires on every move event, incl. during death         | High     | R2     |
| [ ]  | 3   | Falling out of `Seventh` above Y `-128`                | Never triggers; can stand on the `Seventh` floor forever              | Medium   | R1     |
| [ ]  | 4   | Ascending into `First`                                 | Teleports to `-127`, far below its `Y 195–319` range                  | High     | R1     |
| [ ]  | 5   | Descending into any layer                              | Teleports to `318`, above `Second` and inside `First`                 | High     | R1     |
| [ ]  | 6   | Any transition                                         | X/Z preserved but no safe-position fallback; can land in terrain      | Medium   | R1     |
| [ ]  | 7   | Re-entering the layer you just left                    | Destination triggers `up()` on the next move → ping-pong loop         | High     | R1, R2 |
| [ ]  | 8   | Curse applied on a re-entry loop                       | Strain escalates to death within 6 iterations without intent          | High     | R2     |
| [ ]  | 9   | Another `PlayerMoveEvent` queued in the same tick      | Stale `to_position` compared against the wrong world's frame          | Medium   | R3     |
| [ ]  | 10  | `get_world_by_name` returns `None`                     | Silent `return`; player stays in the void, no log                     | Medium   | —      |
| [ ]  | 11  | Layer disabled (`enabled() == false`, all seven today) | Transitions still fire; only the title is gated                       | Low      | —      |
| [ ]  | 12  | Player in a non-Abyss world                            | Correctly ignored                                                     | —        | —      |
| [ ]  | 13  | Manual `/tp` above `MAX_Y`                             | Treated as an ascent and cursed                                       | Medium   | —      |
| [ ]  | 14  | `up()` from `First`                                    | Silent `prev() == None` return; no feedback                           | Low      | —      |
| [ ]  | 15  | Player dies from strain 6 inside the move handler      | Subsequent moves re-trigger transitions on a dead player              | Medium   | R2     |
| [x]  | 16  | Creative / spectator player                            | `layer_leave` skips both (no teleport, no curse)                      | Low      | —      |
| [ ]  | 17  | `Seventh` boundaries                                   | Doc range `Y -64 to -51` is unrelated to `VOID_Y`                     | Medium   | R1     |

## Fixes, in priority order

- [ ] **F1 — Per-layer Y bounds.** Add `Section::floor_y()` / `ceiling_y()` and
      place players inside the target band instead of at global constants.
      Addresses #3–#8, #17. _(R1)_
- [ ] **F2 — One-shot latch.** Track a per-player "in transition" byte (alongside
      the curse strain in persistent data) and skip the handler while set.
      Addresses #1, #2, #7, #8, #15. _(R2)_
- [ ] **F3 — Cancel the move event.** Set `event.cancelled = true` after
      teleporting so the stale position is not processed. Addresses #9. _(R3)_
- [ ] **F4 — Log missing worlds.** `warn!` when `get_world_by_name` fails instead
      of `return`ing silently. Addresses #10.
- [ ] **F5 — Define `enabled()` semantics.** Decide whether it gates transitions
      or display only, and document it. Addresses #11.
- [ ] **F6 — Skip dead (and optionally creative/spectator) players.** Addresses
      #15, #16.
      _Creative and spectator are now skipped by `LayerLeave` (`utils::ignores_transitions`);
      dead-player handling remains._
- [ ] **F7 — Feedback on terminal transitions.** Show a message when leaving
      `First` upward or `Seventh` downward. Addresses #14 (and partly #3).

## Notes

- #12 needs no action; it is listed to record that the guard works.
- `Section::curse()` already derives from the enum order, so F1 should follow the
  same pattern: derive bounds rather than maintaining a second table.
