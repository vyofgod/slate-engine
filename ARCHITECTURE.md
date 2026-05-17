# Slate Engine Architecture

This document describes the current architecture of the Slate workspace as it exists in source. It is intentionally technical and avoids claims that are not backed by the codebase.

For a higher-level overview, start with [README.md](./README.md). For concrete crate-level capability notes, see [FEATURES.md](./FEATURES.md) and [CAPABILITIES.md](./CAPABILITIES.md).

## Architectural Motivation

Most browser engines have many large subsystems that interact through complex state: parsing, style, layout, scripting, networking, painting, compositing, GPU work, storage, events, and platform integration. Slate is organized around a smaller internal contract: transform input into explicit instructions, apply those instructions to state, and render from the resulting primitives.

The important point is not that this automatically makes the engine complete or faster. The point is that it creates a clear place to inspect behavior:

- before dispatch: web-facing input is still high-level
- after dispatch: behavior is represented as AIS
- after kernel application: state can be snapshotted
- after rendering: output artifacts can be checked

That makes the architecture easier to test incrementally.

## System Overview

Slate is structured as a pipeline of explicit transformations:

```text
Input source
  -> parser / runtime / compatibility layer
  -> WebCall
  -> dispatcher
  -> AtomicInstruction stream
  -> kernel
  -> state store + arena
  -> renderer
```

The core architectural choice is to keep the engine’s internal contract narrow:

- `WebCall` is the front-door representation for higher-level actions.
- `AtomicInstruction` is the internal execution form.
- The kernel applies instructions into deterministic state.
- Rendering is a separate headless stage.

## Data Boundaries

Slate has several important boundaries:

| Boundary | Input | Output | Responsibility |
| --- | --- | --- | --- |
| Parser/runtime to dispatcher | web-facing call | `WebCall` | Convert user-facing activity into a normalized internal call. |
| Dispatcher to kernel | `WebCall` | AIS stream | Normalize and decompose without owning global engine state. |
| Kernel to state | AIS stream | snapshot/state mutation | Apply deterministic state changes and expose snapshots. |
| Kernel to renderer | render instructions | frame output | Hand visual primitives to a headless renderer. |

Keeping those boundaries visible helps prevent one subsystem from silently taking over another subsystem’s job.

## Top-Level Layers

### 1. Front-End Sources

These active workspace crates produce or model web-facing behavior:

- `slate-script`
- `slate-network`
- `slate-webapi`
- `slate-html`
- `slate-css`
- `slate-events`
- `slate-dom`

Some of these are compatibility layers or staged surfaces rather than complete browser implementations.

The repository also contains experimental directories for `slate-wasm`, `slate-workers`, `slate-websocket`, `slate-webgl`, and `slate-storage`. They are not root workspace members at the moment, so architecture notes should treat them as staged code rather than active workspace components.

Front-end sources should be treated as producers of data, not as owners of the whole engine. A parser can create tree structures or calls. A compatibility layer can translate API-like operations. A script bridge can emit owned calls. None of those layers should bypass the dispatcher/kernel path when the goal is to exercise the core engine model.

### 2. Translation Layer

`slate-dispatcher` is the central translation layer. Its job is to normalize and decompose web-facing input into AIS.

Key traits visible in source:

- stateless API surface
- deterministic output for identical input
- single-pass decomposition model
- small-stream batch support

The dispatcher is where high-level intent becomes engine work. This is the layer to inspect when a demo emits too many instructions, misses a state mutation, or produces render primitives that do not match expectations.

### 3. Kernel and State

`slate-kernel` is the orchestration point. It owns:

- the deterministic state store
- the page-scoped arena allocator
- instruction submission and replay entry points

`slate-state` provides the state model, while `slate-arena` provides fast page-scoped allocations with O(1) reset semantics.

The kernel is deliberately small in concept: submit calls, replay instructions, expose snapshots, and manage page-lifecycle allocation. That makes it a natural integration point for demos and higher-level tests.

### 4. Rendering

`slate-render` is a headless `wgpu` renderer. It consumes AIS render primitives and writes into an offscreen texture. The renderer is intentionally separate from any windowing layer.

Headless rendering is valuable because it can be tested without a browser shell. The benchmark harness can verify that a PPM file exists, has expected dimensions, and has stable bytes or hashes across runs.

## Crate Responsibilities

### `slate-ais`

Core internal IR for the engine:

- geometry primitives
- render primitives
- state primitives
- domain-tagged atomic instructions

This crate is the shared contract between dispatch, state, and render.

When this crate changes, the blast radius is broad. A new primitive or changed primitive shape can affect dispatch, state application, rendering, demos, and benchmarks.

### `slate-dispatcher`

The dispatcher turns normalized web-facing calls into instruction streams.

Observed structure:

- `normalize`
- `decompose`
- `dispatch`
- `dispatch_batch`

Good dispatcher changes should be easy to validate by checking:

- emitted instruction count
- instruction domain mix
- error behavior for malformed inputs
- compatibility with `Kernel::submit` and `Kernel::submit_batch`

### `slate-state`

The state store is designed around deterministic snapshots and repeatable application of instructions. The source emphasizes predictability over hidden side effects.

State changes should remain replay-friendly. If a behavior depends on wall-clock time, thread scheduling, file system state, or non-deterministic iteration order, it should be isolated and documented.

### `slate-arena`

The arena is page-scoped and intended for short-lived allocations tied to a page lifecycle.

### `slate-kernel`

The kernel owns the integration flow:

- accepts web calls
- translates them into AIS
- applies them to state
- exposes snapshots
- supports replay

### `slate-render`

The renderer uses `wgpu` in headless mode and is structured around batched primitives and readback support.

Rendering should be validated at two levels:

- API level: render commands are accepted and processed
- artifact level: generated image data exists and has expected dimensions

### `slate-text`

Text infrastructure includes shaping, glyph, layout, and rasterizer support code. The crate is structured as a real subsystem, but feature completeness should be verified against code rather than assumed from old docs.

### `slate-css`

CSS support includes parser, selector, cascade, values, and parser integration modules.

### `slate-html`

HTML support includes parser and tree-related modules.

### `slate-events`

The event system models event dispatch, listener registration, and event types.

### `slate-layout`

Layout support includes flexbox, block, inline, and grid components.

Layout code is usually best tested with targeted fixtures. Full pipeline demos are useful, but they can hide whether a regression came from parsing, style, layout, rasterization, or artifact output.

## ASCII Architecture Map

```text
┌─────────────────────────────────────────────────────────────┐
│ Input sources: script, network, html, compatibility layers  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ slate-dispatcher                                             │
│ normalize -> decompose -> instruction stream                │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ slate-kernel                                                 │
│ instruction application -> snapshot -> replay                │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ slate-state + slate-arena                                    │
│ deterministic storage + page-scoped allocation               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ slate-render                                                 │
│ headless wgpu pipeline -> offscreen texture                  │
└─────────────────────────────────────────────────────────────┘
```

## Design Invariants

These are the architectural properties most strongly reflected in the code:

1. Keep the internal instruction surface small.
2. Prefer deterministic transformations over implicit behavior.
3. Separate translation from execution.
4. Keep rendering headless and explicit.
5. Keep page-lifecycle allocation isolated in the arena.

## Failure Modes To Watch

The architecture can drift in a few predictable ways:

- documentation describes inactive crate directories as active workspace crates
- demos succeed while crate APIs remain incomplete
- benchmark docs list commands that no longer exist
- compatibility layers start claiming browser-standard completion without tests
- render artifacts are generated but not verified
- state transitions become dependent on environment details

The current documentation intentionally calls out those risks so future changes can be reviewed against them.

## Validation Strategy

Useful validation layers:

- `cargo check --workspace` for compile coverage over active workspace members
- targeted crate tests for local behavior
- Criterion benchmarks for micro and crate-level measurements
- `scripts/browser_engine_benchmark.py` for demo pipeline measurement and artifact validation
- manual review of generated docs and reports before publishing

## Subsystem Contracts

Each subsystem should have a clear contract. These contracts are not formal Rust traits in every case; they are engineering boundaries the code and docs should preserve.

| Subsystem | Owns | Should not own |
| --- | --- | --- |
| AIS | instruction vocabulary and primitive types | parsing policy, DOM semantics, network behavior |
| Dispatcher | normalization and decomposition | global state, rendering lifecycle, async runtime ownership |
| State | deterministic store and snapshots | parsing, GPU submission, network access |
| Arena | page-scoped allocation | long-lived persistent storage, observable state semantics |
| Kernel | orchestration and replay | browser UI, standards compatibility claims |
| Renderer | offscreen rendering and readback | DOM mutation, CSS parsing, script execution |
| HTML/CSS/DOM/Layout | document modeling and visual computation | GPU device lifecycle, benchmark reporting |
| Web API compatibility | adapter and translator surfaces | full browser implementation claims |

If a change violates one of these boundaries, it may still be correct, but the architecture document should be updated to explain why the boundary moved.

## Instruction Lifecycle

The lifecycle of an operation should be inspectable at several points:

1. A high-level source creates or implies a web-facing action.
2. The action is represented as a borrowed or owned call.
3. The dispatcher validates and normalizes that call.
4. The dispatcher decomposes it into AIS.
5. The kernel applies the AIS to state.
6. Render primitives are passed to rendering code when applicable.
7. Demos or benchmarks record stdout, metrics, and artifacts.

This lifecycle gives contributors multiple debugging hooks. If a visual output is wrong, inspect whether the source emitted the right call, whether dispatch produced the right primitive, whether state changed correctly, and whether the renderer produced the expected artifact.

## Determinism Model

Slate’s current docs use “deterministic” as an architectural preference, not as a blanket proof for all code. The intended deterministic core is:

- the same normalized input should produce the same instruction stream
- replaying the same instruction stream should produce the same state result
- demo output should be stable enough to compare across local runs when environment conditions are unchanged

Things that can weaken determinism:

- GPU adapter differences
- non-deterministic map iteration
- wall-clock timestamps
- random data
- file system state
- network timing
- async scheduling

When any of those are necessary, isolate them at the boundary and avoid letting them leak into core state semantics.

## Rendering Architecture Notes

Rendering currently has two important forms in the repository:

- `slate-render` for headless `wgpu` rendering
- `slate-rasterizer` for CPU display-list style output

Both are useful, but they answer different questions. CPU raster output is easier to verify in simple demos because the generated artifact is deterministic and easy to inspect. GPU rendering is closer to the intended high-performance path, but it depends on local adapter availability.

The benchmark harness handles this by recording whether a GPU path was skipped and by validating generated PPM artifacts when they exist.

## Workspace Boundary Notes

The root workspace is the source of truth for active crates. A directory under `crates/` is not automatically active. This matters for architecture because inactive directories:

- may not compile under normal workspace commands
- may depend on older dependency versions
- may not be covered by benchmark or test workflows
- may contain useful design sketches without being productionized

When a staged directory becomes active, update these files together:

- root `Cargo.toml`
- [README.md](./README.md)
- [CAPABILITIES.md](./CAPABILITIES.md)
- [FEATURES.md](./FEATURES.md)
- [BENCHMARKS.md](./BENCHMARKS.md), if it introduces benchmarkable behavior

## Review Checklist For Architecture Changes

- Does the change keep input translation separate from state application?
- Does the change keep rendering separate from parsing and DOM mutation?
- Does the change require a new AIS primitive?
- Does an existing primitive already express the behavior?
- Does the change affect replay or snapshot behavior?
- Does the change make a staged crate part of the active workspace?
- Does benchmark documentation need an update?
- Does the README still describe the repository accurately?

## Cross-References

- [README.md](./README.md) for project entry points and repository map.
- [FEATURES.md](./FEATURES.md) for capability-by-capability status.
- [CAPABILITIES.md](./CAPABILITIES.md) for a concise component summary.
- [BENCHMARKS.md](./BENCHMARKS.md) for measurement targets and commands.
- [ROADMAP.md](./ROADMAP.md) for planned work.
