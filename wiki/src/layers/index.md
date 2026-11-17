# The Abyss

Mine in Abyss is built as a **vertical world**. Each layer is its own Minecraft world, and you
move between them by descending past the bottom or climbing past the ceiling.

<div class="card-grid">

<a class="card" href="depths.html">
<span class="card-header">
<span class="card-title">The Layers</span>
</span>
<span class="card-desc">All eight layers, surface to Final Maelstrom</span>
</a>

<a class="card" href="curse.html">
<span class="card-header">
<span class="card-title">Curse of the Abyss</span>
</span>
<span class="card-desc">What ascending costs you, and how it compounds</span>
</a>

<a class="card" href="travel.html">
<span class="card-header">
<span class="card-title">Descending & Ascending</span>
</span>
<span class="card-desc">How to move between layers, and what follows you</span>
</a>

</div>

## The Shape of the Abyss

Every layer is a full-height world of its own, 384 blocks tall. The bottom of each layer and the
top of the next are the same boundary — fall out of one and you arrive at the top of the layer
below.

| Boundary | Behaviour |
| -------- | --------- |
| Below Y `-64` | You fall into the next layer down, arriving near its ceiling |
| Above Y `319` | You climb into the previous layer up, arriving near its floor |

Nothing physically connects the layers. There is no shaft to walk down and no ladder to climb —
the transition is handled by the server the moment you cross the boundary.
