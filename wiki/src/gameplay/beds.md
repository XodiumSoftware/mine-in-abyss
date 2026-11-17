# Beds & Spawning

## You Cannot Set a Spawn Point

**Beds and respawn anchors do not work.** Using one shows a short red message and nothing is
stored — there is no spawn point to return to.

This is deliberate. A bed at the bottom of the Abyss would let you skip the entire climb — and
the Curse that comes with it — simply by dying there. The Abyss is meant to be escaped on its own
terms.

| Action | Result |
| ------ | ------ |
| Sleeping in a bed | Refused |
| Respawn anchor | Refused |
| `/spawnpoint` | **Works** (operators only) |
| Clearing a spawn point | Works — removing one is always allowed |

The difference is who asked. A bed is a player shortcut; `/spawnpoint` is an operator decision,
so it keeps working.

## Where You Respawn

**Death always returns you to Orth's world spawn**, no matter which layer you died in.

This is not just a position change — it is a different world. So even if the Curse kills you at
the bottom of the Abyss, you wake up in the City on the Edge with your strain cleared.

See [Curse of the Abyss](../layers/curse.md) for what dying does to your strain.
