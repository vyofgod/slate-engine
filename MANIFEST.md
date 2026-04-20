# The Slate Manifest

> *"Do not interpret the web; translate and dominate it."*

## 1. The Problem We Refuse To Inherit

Every mainstream engine — Blink, WebKit, Gecko — spends the majority of its
execution budget *negotiating between its own layers*. A `<div>` is not a
rectangle to them; it is an object subject to hundreds of overlapping APIs
(CSSOM, layout, compositor, accessibility tree, input routing, devtools
hooks) each of which may observe or mutate the others. The engine spends
most of its cycles resolving this polyphony. We call this the
**interpretation tax.**

Slate does not pay it.

## 2. The Principle: API Elimination via Atomic Reduction

The web platform exposes tens of thousands of surface points. Slate's claim
is that all of them reduce — *without loss of observable behavior* — to a
closed set of **200–500 Atomic Primitives** spread across three domains:

| Domain    | Role                                                         |
|-----------|--------------------------------------------------------------|
| Layout    | Pure geometry. Sub-pixel, SIMD-friendly, tree-independent.   |
| Render    | GPU command-buffer ops. Zero-copy to Vulkan/Metal/WebGPU.    |
| State     | Purely functional state deltas. Replayable. Deterministic.  |

The engine executes **only** these primitives. Every high-level call
(`document.createElement`, `fetch`, a CSS Grid layout, a keyframe
animation) is decomposed to this form **before** it reaches the kernel.

## 3. The Architecture

```
 ┌──────────────────────────────────────────────────────────────┐
 │   Legacy Web Chaos  (HTML / CSS / JS / Web APIs)             │
 └──────────────────────────────────────────────────────────────┘
                             │
                             ▼
 ┌──────────────────────────────────────────────────────────────┐
 │   Dispatcher  —  stateless translation bridge (Wine-like)     │
 │   (a) Intercept  (b) Normalize  (c) Decompose  (d) Inline     │
 └──────────────────────────────────────────────────────────────┘
                             │
                             ▼ stream of AtomicInstruction
 ┌──────────────────────────────────────────────────────────────┐
 │   Kernel  +  Deterministic State Store                       │
 │   slotmap · dashmap · crossbeam · immutable snapshots         │
 └──────────────────────────────────────────────────────────────┘
                             │
                             ▼
 ┌──────────────────────────────────────────────────────────────┐
 │   GPU Pipeline  (Vulkan / Metal / WebGPU)                    │
 └──────────────────────────────────────────────────────────────┘
```

## 4. Invariants (Non-Negotiable)

1. **Closure under decomposition.** If a Web call cannot be reduced to AIS,
   it is not supported. The bridge never falls back to a "legacy path."
2. **Determinism.** `(snapshot, input_sequence) → snapshot'` is a pure
   function. No wall clock, no thread scheduling, no allocator address
   escapes into observable state.
3. **Single-pass dispatch.** The Dispatcher produces the AIS stream in
   O(n) over the input call sequence. No fix-point, no retry, no layout
   thrashing.
4. **Zero-copy to GPU.** Render primitives are laid out with `repr(C)` and
   sized for direct upload. No intermediate paint buffer on the CPU.
5. **No GC.** Per-page state lives in a `bumpalo` arena; reset is O(1) on
   navigation. `unsafe` is permitted only on paths with benchmark
   justification recorded in the PR.

## 5. What This Buys Us

- **Security:** attack surface collapses with API surface. A primitive has
  no parser, no callback, no prototype chain to pollute.
- **Speed:** the kernel executes instructions, not intentions. Layout
  passes are embarrassingly parallel because primitives are pure over
  geometry.
- **Replayability:** any bug can be reproduced from `(snapshot, inputs)`.
  Debugging becomes mechanical.
- **Footprint:** the engine fits on hardware that cannot run Chromium.

## 6. North Star For Contributors

Before adding code, ask: *does this re-introduce abstraction bloat?*

If the answer is "it is more ergonomic / it matches the spec more
literally / it is how Chromium does it" — the patch is rejected. If the
answer is "it shrinks the primitive set, tightens determinism, or removes
a CPU-bound phase" — it is welcome.

This document is the contract. The code is downstream of it.

---

## Addendum: Phase 2 — Execution & Visualization

Phase 1 produced the AIS stream. Phase 2 makes it **visible** and
**scriptable** without weakening any of the invariants above.

### Three new subsystems

- **`slate-render`** — wgpu-based. Consumes `&[AtomicInstruction]` and
  issues a single instanced draw per frame for all `FillRect`/
  `StrokeRect` primitives. No CPU paint phase, no backing bitmap.
  The offscreen target is Rgba8UnormSrgb; readback is padded-row aware.
  `FrameScheduler` keeps the cadence (e.g. 300 Hz) on the kernel's
  clock, not the compositor's.

- **`slate-script`** — Boa `Context` on an isolated thread. The only
  thing JS can do is call three host functions (`__slate_create_element`,
  `__slate_append_child`, `__slate_set_style`) which push
  `OwnedWebCall`s into a thread-local buffer. The kernel drains that
  buffer on each frame. JS can't touch the state store. JS can't see
  wall time. Determinism holds.

- **`slate-network`** — tokio + reqwest. The streaming fetcher returns
  chunks as `reqwest::Response::bytes_stream`; an `IncrementalParser`
  consumes those chunks and emits `OwnedWebCall`s the moment a tag
  closes. First AIS instructions can reach the renderer before the
  HTML body is fully downloaded. An `OriginPolicy` guards every
  request; process sandboxing is Centrion's job.

### Load-bearing types

- `OwnedWebCall` — lifetime-free twin of `WebCall<'a>` with a zero-copy
  `.as_web_call()` adapter. Required for crossing async/thread/script
  boundaries without bumping into borrowck.
- `Kernel::submit_owned` / `submit_batch` — the single integration
  seam between the three new subsystems and the Phase 1 Dispatcher.

### Still true

The AIS surface did **not** grow in Phase 2. Everything the renderer,
script bridge, and network layer produce goes through the same
primitives defined in `slate-ais`. That is the test: if adding a
subsystem forced a new primitive, we failed. It didn't. We held the
line.
