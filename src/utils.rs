//! Small helpers shared across the plugin's events and layers.
//!
//! Split by what each module owns rather than by which handler uses it: the three
//! `thread_local` records a transition needs are each a module of their own, so the state
//! and the functions that maintain it cannot drift apart.
//!
//! Everything is re-exported from this file's parent module, so a caller still writes
//! `use crate::utils::{carry_fall, layer_of};` and does not have to know which file a
//! helper ended up in. Reach for `utils::fall::…` directly only when the grouping is what
//! you are looking for.
//!
//! The submodules and their re-exports are declared in the `mod utils` block in `lib.rs`,
//! the same way `commands` and `events` are.
