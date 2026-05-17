# Slate Engine Capabilities

This document is a concise, source-aligned overview of the workspace. It links back to [README.md](./README.md) and [ARCHITECTURE.md](./ARCHITECTURE.md) for the broader context.

## How To Read This File

This file separates three ideas that are easy to blur:

- active workspace member: included by root `Cargo.toml` and covered by workspace commands
- source directory: present in the repository, but not necessarily part of the active workspace
- completed feature: behavior with enough implementation and tests to rely on as a stable capability

The check marks below mean active or present in the stated category. They do not mean standards-complete browser behavior.

## Core Engine

- [x] Atomic instruction layer in `slate-ais`
- [x] Stateless dispatcher in `slate-dispatcher`
- [x] Deterministic state store in `slate-state`
- [x] Page-scoped arena in `slate-arena`
- [x] Kernel orchestration in `slate-kernel`
- [x] Headless renderer in `slate-render`

Core engine capabilities are the most important part of the current project. They define the flow from calls into instructions, state, and rendering. If these pieces fail to compile or drift apart, higher-level compatibility work becomes unreliable.

## Parsing and Model Layers

- [x] HTML parsing support in `slate-html`
- [x] CSS parsing and selector matching in `slate-css`
- [x] DOM model and mutation tracking in `slate-dom`
- [x] Event dispatch model in `slate-events`
- [x] Layout engines in `slate-layout`

These crates provide the vocabulary for web-like documents and behavior. Their existence does not imply that every HTML parser state, CSS property, DOM edge case, or layout rule has complete browser parity.

## Runtime and Compatibility Layers

- [x] JavaScript runtime bridge in `slate-script`
- [x] Network fetch and streaming parser support in `slate-network`
- [x] Web API compatibility crate in `slate-webapi`

These layers are where external or web-facing behavior starts to enter the engine. They should be validated through integration tests and the Python benchmark harness when their output affects the kernel or renderer.

## Experimental Crate Directories

These directories exist in the repository but are not active root workspace members yet:

- [ ] `slate-storage`
- [ ] `slate-wasm`
- [ ] `slate-webgl`
- [ ] `slate-websocket`
- [ ] `slate-workers`

These directories may contain meaningful code, but normal workspace commands do not currently prove them. Before presenting one of them as an active capability, add it to root workspace membership and run the relevant checks.

## Graphics and Media

- [x] `wgpu`-based headless render path
- [x] CPU rasterization utilities
- [x] Image loading/decoding crate in the active workspace
- [x] Text rendering and layout crate in the active workspace

Graphics and media behavior should be described narrowly. For example, “image support crate exists” is safer than “all image formats are supported.” The former is source-aligned; the latter needs format-specific tests.

## Tooling and Demos

- [x] Demo binaries in `slate-kernel`
- [x] Benchmark harnesses
- [x] Example HTML page in `examples/`
- [x] Static documentation in `docs/`

The demos are useful for exercising pipeline behavior, but they are not compliance tests. Treat them as smoke tests and benchmark targets.

## Capability Review Checklist

Before adding a capability claim to this file, check:

- Is the crate in root `Cargo.toml` workspace members?
- Does `cargo check --workspace` cover it?
- Is there a demo, test, or benchmark that exercises the behavior?
- Does the claim describe exact source behavior rather than future intent?
- Would a reader confuse the claim for full browser-standard support?

## Notes

The check marks above indicate that the corresponding crate or subsystem is part of the active source/workspace surface. They do not imply every API surface is complete, final, or production-hardened.

For the current implementation details, use:

- [FEATURES.md](./FEATURES.md)
- [BENCHMARKS.md](./BENCHMARKS.md)
- [ROADMAP.md](./ROADMAP.md)

## Capability Matrix

| Area | Active workspace? | Demo exercised? | Benchmark coverage? | Notes |
| --- | --- | --- | --- | --- |
| AIS | Yes | Yes, through `slate-demo` | Indirect through harness and Criterion | Core instruction vocabulary. |
| Dispatcher | Yes | Yes, through `slate-demo` and `slate-pipeline` | Yes | Central translation layer. |
| State store | Yes | Yes | Indirect | Snapshot and replay behavior should remain conservative. |
| Arena | Yes | Indirect | No direct high-level benchmark | Page-lifecycle allocation support. |
| Kernel | Yes | Yes | Yes | Main integration point. |
| Headless GPU rendering | Yes | Yes, through `slate-pipeline` | Yes, environment-dependent | Can skip or differ if no compatible adapter is present. |
| CPU raster output | Yes | Yes, through `slate-phase2` | Yes, via artifact validation | Produces PPM output in demo path. |
| HTML parsing | Yes | Yes, through `slate-phase2` | Yes, as part of pipeline | Do not treat as full browser compliance by default. |
| CSS parsing/cascade | Yes | Yes, through `slate-phase2` | Criterion and pipeline | Coverage depends on properties/selectors exercised. |
| DOM model | Yes | Yes, through `slate-phase2` | Pipeline-level | Useful for mutation and dirty-state flows. |
| Layout | Yes | Yes, through `slate-phase2` | Criterion and pipeline | Needs fixture tests for deeper confidence. |
| Text | Yes | Not fully exercised by every demo | Limited | Describe narrowly unless covered by tests. |
| Image | Yes | Yes, through `slate-phase4-demo` | Pipeline harness path | Format-specific support should be verified separately. |
| Web API compatibility | Yes | Yes, through `slate-phase4-demo` | Pipeline harness path | Translation-oriented, not full platform parity. |
| Storage | No | No active workspace demo | No active workspace benchmark | Experimental directory. |
| WASM | No | No active workspace demo | No active workspace benchmark | Experimental directory. |
| WebGL | No | No active workspace demo | No active workspace benchmark | Experimental directory. |
| WebSocket | No | No active workspace demo | No active workspace benchmark | Experimental directory. |
| Workers | No | No active workspace demo | No active workspace benchmark | Experimental directory. |

## Capability Claim Language

Prefer these forms:

- “The active workspace includes `slate-html`.”
- “The `slate-phase2` demo exercises HTML parsing, CSS parsing, DOM construction, layout, display-list construction, and CPU raster output.”
- “The Python benchmark harness validates generated PPM artifacts for applicable scenarios.”
- “The repository contains an experimental `slate-webgl` directory that is not currently a root workspace member.”

Avoid these forms unless backed by tests and compatibility targets:

- “Slate supports HTML5.”
- “Slate has complete WebGL.”
- “Slate is production-ready.”
- “Slate is faster than Chromium.”
- “All Web APIs are implemented.”

## Promotion Criteria For Experimental Directories

Before moving an experimental directory into the active capability list:

1. Add it to root workspace members.
2. Ensure `cargo check --workspace` covers it.
3. Ensure dependency versions do not conflict with the active workspace.
4. Add at least one direct test or demo path.
5. Add benchmark coverage if it affects runtime behavior.
6. Update README, capabilities, features, and architecture docs together.
7. Remove language that describes it as experimental only after verification succeeds.
