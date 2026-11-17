# Descending & Ascending

Layers are connected vertically. Crossing a boundary is handled by the server — you keep your
X and Z coordinates, and the server picks the Y you arrive at.

## Descending

Fall past the bottom of a layer and you drop into the one below.

| What happens        | Detail                                           |
| ------------------- | ------------------------------------------------ |
| Arrival height      | Just below the ceiling of the layer below        |
| Horizontal position | Unchanged — the same X and Z                     |
| Curse               | **None.** Descending is always free              |
| Fall distance       | **Carried.** The drop continues in the new layer |

Because each layer is only 384 blocks tall, a fall that started in the layer above would
otherwise never reach a damaging height. The server carries the distance across the boundary,
so the impact at the bottom is what you would have taken had the Abyss been one continuous
drop.

**Only one layer is carried.** Fall from the First without landing and you take the damage of
the last segment, not the whole descent.

## Ascending

Climb past the top of a layer and you enter the one above.

| What happens        | Detail                                                |
| ------------------- | ----------------------------------------------------- |
| Arrival height      | Just above the floor of the layer above               |
| Horizontal position | Unchanged — the same X and Z                          |
| Curse               | **Applied**, equal to the depth of the layer you left |
| Monsters            | Left behind                                           |

See [Curse of the Abyss](curse.md) for what the strain does to you.

## What Does Not Transfer

- **Creatures stay where they are born.** Nothing follows you between layers, which cuts both
  ways: what was chasing you cannot follow you down, and your surroundings are entirely the new
  layer's.
- **Items on the ground stay put.** Entities cannot change layers, so a dropped item or a
  thrown mob that falls into the void simply falls to the world floor.

## Operators and Teleports

An operator teleport is not a climb. If an operator moves you upward, you are not cursed for it —
the server recognises that the next position was a teleport rather than a jump, and ignores it.

A real climb immediately afterwards is still cursed. The exception covers the teleport itself,
not the rest of your journey.
