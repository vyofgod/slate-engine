# Slate Engine Feature Overview

This document summarizes the feature areas that are represented in source today. For the project map, see [README.md](./README.md). For architecture, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## Purpose

This is a descriptive file, not a marketing feature list. It answers: “Which kinds of functionality are represented in the current source tree, and what should a contributor inspect first?”

Use [CAPABILITIES.md](./CAPABILITIES.md) when you need the shorter status view. Use this file when you want more context about the feature areas.

## Present in Source

### Atomic Instruction System

`slate-ais` defines the shared instruction model used by the rest of the engine.

- geometry primitives
- render primitives
- state primitives
- domain-tagged atomic instructions
- compact stream representation

The AIS layer is the internal language shared by the engine. When a high-level action is translated successfully, it should become a sequence of these primitives. That instruction stream is what makes pipeline output inspectable.

### Dispatch

`slate-dispatcher` provides:

- call normalization
- translation into AIS
- batch dispatch support
- error reporting for unsupported or malformed inputs

The dispatcher is a good place to add instrumentation because it is close to the point where high-level behavior becomes concrete work. Instruction counts and domain breakdowns are useful signals in benchmarks.

### State and Allocation

`slate-state` and `slate-arena` provide:

- deterministic state handling
- snapshot support
- page-scoped allocation
- reset-on-navigation semantics

This area is responsible for making repeated engine runs predictable. The page arena is especially important for lifecycle behavior: temporary page data should be easy to discard on navigation or reset.

### Rendering

`slate-render` provides a headless `wgpu` path with:

- offscreen target creation
- buffer setup
- primitive batching
- pixel readback

The current renderer is headless. That keeps it testable from command-line benchmarks and avoids coupling it to a specific windowing shell.

### HTML, CSS, DOM, Layout

The repository includes source for:

- HTML parsing
- CSS parsing and cascade
- DOM mutation and querying
- event dispatch
- flexbox, block, inline, and grid layout

These are active areas where “feature exists” should be read carefully. A crate can contain a parser, cascade engine, or layout implementation without covering every browser-standard edge case.

### Text, Image, and Network

The repository also contains:

- text shaping and layout scaffolding
- image decoding/loading scaffolding
- incremental network fetching and sandbox hooks

These systems are important for realistic page behavior. They should be benchmarked with real artifacts and real input data rather than only isolated unit calls.

### Runtime and Platform Surfaces

Additional active workspace crates cover:

- JS runtime bridging
- Web API compatibility translation

The repository also contains experimental directories for WebGL, WebSocket, WASM, workers, and storage. They are present in source but are not currently root workspace members.

## Feature Maturity Categories

Use these labels when expanding the docs:

| Label | Meaning |
| --- | --- |
| Active workspace surface | Included in root workspace and covered by workspace commands. |
| Demo-exercised | Covered by one of the runnable demos or the Python benchmark harness. |
| Experimental directory | Present in the repository but not a root workspace member. |
| Planned | Described as future work only. |

Avoid using “complete” unless there is a clear test suite and compatibility target backing that statement.

## What This Does Not Mean

The presence of a crate does not automatically mean:

- the full browser-standard API is complete
- the implementation is production hardened
- all modules are equally mature
- every documented surface in older notes is still accurate

## Useful Inspection Points

- `crates/slate-ais/src/lib.rs` for the instruction model
- `crates/slate-dispatcher/src/lib.rs` for dispatch entry points
- `crates/slate-kernel/src/lib.rs` for orchestration
- `crates/slate-render/src/lib.rs` for headless rendering
- `scripts/browser_engine_benchmark.py` for end-to-end demo measurement

## Feature Area Details

### Core Instruction Features

The AIS feature area is the foundation for everything else. It should remain small, stable, and easy to audit. Additions to AIS should be reviewed carefully because they tend to create follow-up work in dispatch, state application, renderer handling, tests, and documentation.

Questions to ask before extending AIS:

- Is the behavior truly atomic?
- Can it be represented as a sequence of existing primitives?
- Which subsystem will consume the primitive?
- Does it require a state transition, render operation, layout operation, or all of them?
- How will it be measured or tested?

### Dispatch Features

Dispatch features are about converting calls into explicit work. Good dispatch behavior should be predictable, easy to count, and easy to reject when malformed.

Useful signals:

- number of emitted AIS instructions
- domain split between layout, render, and state
- error type for unsupported behavior
- whether batch dispatch preserves ordering

### Document Model Features

HTML, CSS, DOM, and layout features form the document processing surface. These features should be documented with care because browser standards contain many edge cases.

Safer documentation pattern:

- “The crate includes selector matching logic.”
- “The demo parses a stylesheet with several rules.”
- “The layout crate contains flexbox, block, inline, and grid modules.”

Riskier documentation pattern:

- “CSS is fully supported.”
- “HTML5 is complete.”
- “Layout matches browser behavior.”

### Runtime Features

Runtime features include scripting and compatibility layers. These should be validated by checking what calls they emit and how those calls behave after dispatch.

Important questions:

- What host functions or compatibility functions are active?
- What owned calls are produced?
- Does the kernel accept those calls?
- Are unsupported operations explicit?

### Rendering Features

Rendering features should be tied to artifact output wherever possible. A visual feature is much easier to trust when a benchmark or demo can write an output file and report its dimensions and hash.

For render-related changes, update or inspect:

- demo output paths
- artifact validation in the Python harness
- renderer error handling
- GPU skip behavior
- `.gitignore` entries for generated files

## Feature Documentation Template

When documenting a new feature, use this shape:

```md
### Feature Name

Status: active workspace surface / demo-exercised / experimental directory / planned.

Source:
- `path/to/source.rs`

Validation:
- `command to run`

Notes:
- Exact behavior that exists today.
- Known limitations.
- What should not be inferred.
```

This keeps feature docs useful without turning them into broad claims.

## Related Documents

- [CAPABILITIES.md](./CAPABILITIES.md)
- [BENCHMARKS.md](./BENCHMARKS.md)
- [ROADMAP.md](./ROADMAP.md)
