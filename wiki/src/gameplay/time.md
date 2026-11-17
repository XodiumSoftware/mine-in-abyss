# Time of Day

Every layer is a separate world, and every world keeps its own clock. Left alone they drift
apart, so a player who descended would arrive at a different time of day than the one they left —
the sun would jump mid-fall.

## How It Works

**Orth's clock is authoritative.** Once a minute, the server copies it to the other seven layers.

| What is shared | Detail |
| -------------- | ------ |
| Time of day | Copied from Orth |
| Day number | Copied too — `/time set 24000` in Orth puts every layer on day 1 |
| World age | **Not shared** — the server has no way to set it |

## Why Orth

Orth is the city above the Abyss: the layer an operator is most likely to change the time in, and
the one every descent starts from. Change the time there and the rest of the Abyss follows within
a minute.

## Notes

- **The task runs once immediately on load**, so a player joining right after a restart does not
  see a stale sun.
- **Two layers can disagree by up to a minute.** That is far below what anyone can perceive, and
  it keeps the cost of syncing off the tick budget.
- **Nothing in vanilla depends on a world's age** matching its time of day, so not sharing age
  has no effect on gameplay.
