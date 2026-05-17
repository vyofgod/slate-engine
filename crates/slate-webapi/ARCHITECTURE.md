# Slate Web API Architecture

This note explains how `slate-webapi` fits into the broader workspace.

See also:

- [Root README](../../README.md)
- [Root Architecture](../../ARCHITECTURE.md)
- [Web API README](./README.md)

## Role in the Workspace

`slate-webapi` sits above the dispatcher and below web-facing input sources. Its job is to translate compatibility-oriented calls into data that the engine can process.

It should be treated as an adapter layer. The dispatcher remains responsible for AIS decomposition, and the kernel remains responsible for state application.

## High-Level Flow

```text
JavaScript or compatibility input
  -> webapi translator
  -> WebCall-like internal representation
  -> dispatcher
  -> AtomicInstruction stream
  -> kernel
```

## Source Reality

The source currently shows a smaller active public surface than some of the older documentation suggested. This document should therefore be used as a role description, not as a promise of full browser parity.

## Design Notes

The crate should stay explicit about what it owns:

- compatibility-facing types and helpers
- translation-oriented modules
- focused API surfaces such as canvas, forms, and SVG helpers

It should avoid owning:

- global engine state
- rendering lifecycle
- network transport policy
- browser UI behavior
- claims of standards completion without tests

## Integration Boundaries

`slate-webapi` should integrate through explicit data flow:

1. A compatibility-facing operation is represented in Rust.
2. The module converts it into internal data or calls.
3. The dispatcher/kernel path handles execution semantics.
4. Demos or tests prove the path works.

If a module starts mutating global engine state directly, it should be reviewed carefully. Direct state mutation makes it harder to reason about replay and benchmark output.

## Documentation Boundaries

This crate’s docs should avoid broad browser-platform wording. Prefer exact statements:

- “`forms` contains validation helpers.”
- “`canvas` contains canvas-oriented compatibility helpers.”
- “`svg` contains SVG-oriented helpers.”
- “`translator` is the central adapter module.”

Avoid broad statements:

- “Slate implements Forms.”
- “Slate supports Canvas.”
- “Slate supports SVG.”
- “Slate implements Web APIs.”
