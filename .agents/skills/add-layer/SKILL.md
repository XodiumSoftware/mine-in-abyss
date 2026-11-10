---
name: add-layer
description: Scaffolds a new mine-in-abyss layer by adding a Section variant, a Layer impl, and registering it in LAYERS.
---

# Add a Layer

Every layer of the Abyss is a struct implementing the `Layer` trait in
`src/layers/layer.rs`. Layers are zero-sized stubs that declare _which_ section they are;
display data lives on `Section`.

**Discover, don't assume.** Read `src/layers/layer.rs`, `src/section.rs` and
`src/layers/first.rs` before writing anything. There is **no config system** — an earlier
version of this skill described `src/config.rs`, `enabled()` gates and a `cmds()`/`perms()`
registration trait, none of which exist.

## The trait

```rust
pub trait Layer {
    fn id(&self) -> Section;
}
```

That is the whole required contract. Names and palettes are `Section::name()` and
`Section::color()`, so they are never repeated in a layer module.

## Steps

1. **`src/section.rs`** — add the variant to the `Section` enum, in depth order
   (`next`/`prev`/`index`/`curse` all derive from variant order). Then fill in its arm in:
    - `Section::ALL`
    - `Section::name()`
    - `Section::color()`

2. **`src/layers/{layer}.rs`** — new module:
    - `//!` docs describing the layer and what belongs there
    - `use crate::layers::layer::Layer;` and `use crate::section::Section;`
    - `#[derive(Debug, Default, Clone, Copy)] pub struct {Layer};`
    - `impl Layer for {Layer} { fn id(&self) -> Section { Section::{Layer} } }`

3. **`src/lib.rs`** — declare the module in the `mod layers` tree and add it to `LAYERS`
   **in depth order**.

4. **Event handlers**, if the layer needs behaviour: implement `EventHandler<T>` in the
   layer module and register it from `MineInAbyss::register_events`. Prefer adding the
   handler once for all layers (as `LayerLeave` does) over per-layer registration.

## Constraints

- **No `//` line comments.** Explain intent with `///` and `//!`, or in names.
- Modules are ordered: `//!`, `use`, `const`, `struct`/`enum`, `impl Layer`,
  `impl EventHandler<T>`, other impls.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- Do not repeat a layer's display name or palette in its module; it belongs to `Section`.
- Each layer world is a **hand-built map**. Do not assume ground at any coordinate, and
  never regenerate layer world data on startup.

## Verification

```bash
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
cargo build --target wasm32-wasip2
cargo doc --no-deps --target wasm32-wasip2
```

Then update `AGENTS.md` (§ Active Layers, § Adding a New Layer) if the tree changed.

After finishing, summarize the files changed and ask the user whether to commit.
