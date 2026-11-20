---
name: update-docs
description: Updates mine-in-abyss rustdoc comments, AGENTS.md, GUIDE.md and ARCHITECTURE.md. Only use when the user explicitly asks for docs to be updated or regenerated.
---

# Update Docs

**Only run this when the user asks.** `AGENTS.md` forbids updating docs as part of a code
change, and most changes need no doc work at all.

## When a doc edit is justified

Write a `//!` or `///` note only when the *reason* is invisible in the code:

- a non-obvious constraint or workaround, such as a host bug or an API gap
- a number that is a guess rather than data, so a reader does not trust it
- ordering that matters, such as why a latch is released in a different handler

Do **not** restate what the source already says. No tables of ids, names, layers or stats —
those are read from the code. If a doc and the code disagree and the doc would mislead, fix
the doc; otherwise leave it alone.

## Conventions

- Module docs use `//!`, items `///`. No `//` line comments anywhere.
- There is **no config system**, so no config tables.
- There is **no `MiniMessage`**; messages are `TextComponent`s built in code.
- `PUBLISHED` facts — trades, drops, prices — are recorded in `wiki/src/`. Do not copy them
  into rustdoc or `AGENTS.md`.
- Read `PLUGIN_ID` and ids from the code rather than restating them from memory.

## Steps

1. Ask what the user actually wants in scope: a module, the top-level docs, or everything.
2. Read the code you are documenting — never document from `AGENTS.md` or this skill alone.
3. If rustdoc changed, confirm it still builds and that no public item links to a private one:
   `just doc`.
4. Confirm lints and formatting still pass: `just lint` and `just fmt-check`.

Report what changed and ask whether to commit. Never commit or push unprompted.
