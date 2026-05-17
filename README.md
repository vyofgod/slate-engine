# Slate Engine

Slate Engine is a Rust workspace for a browser-engine style pipeline built around a narrow atomic instruction layer. The active Cargo workspace is organized as focused crates covering parsing, dispatch, state, layout, rendering, scripting, networking, and compatibility surfaces.

This project is not presented as a finished browser. The documentation in this repository is written to describe the current codebase accurately, without promising capabilities that are not present in source.

## Project Intent
eksi
Slate is an engine research and implementation workspace. Its main idea is to make the internal execution path explicit: high-level input should be reduced into a small, inspectable instruction stream before state mutation or rendering happens.

That gives the project a few concrete engineering goals:

- make the web-facing surface separate from the execution model
- keep the internal instruction format small enough to inspect and test
- preserve deterministic state transitions where possible
- keep rendering isolated from parsing and state mutation
- make demos and benchmarks exercise real code paths instead of only synthetic loops

The project is useful as a place to experiment with browser-engine architecture, not as a drop-in replacement for Chromium, Firefox, WebKit, or a production WebView.

## Documentation Map

- [Architecture](./ARCHITECTURE.md)
- [Capabilities](./CAPABILITIES.md)
- [Features](./FEATURES.md)
- [Benchmarks](./BENCHMARKS.md)
- [Roadmap](./ROADMAP.md)
- [Web API compatibility notes](./crates/slate-webapi/README.md)

Recommended reading path:

1. Start here to understand the repository shape.
2. Read [ARCHITECTURE.md](./ARCHITECTURE.md) for the pipeline and crate boundaries.
3. Read [CAPABILITIES.md](./CAPABILITIES.md) to distinguish active workspace crates from experimental directories.
4. Read [BENCHMARKS.md](./BENCHMARKS.md) before making performance claims.
5. Read [ROADMAP.md](./ROADMAP.md) for future work and priorities.

## Execution Model

Slate follows a direct, explicit pipeline:

```text
Web-facing input
  -> parser / runtime / compatibility layer
  -> WebCall
  -> slate-dispatcher
  -> AtomicInstruction stream
  -> slate-kernel
  -> slate-state + slate-arena
  -> slate-render
```

The design goal is to keep the intermediate instruction surface small and deterministic so the engine is easier to reason about, benchmark, and extend.

In practical terms, the execution path is split into three roles:

- front-end code produces web-facing events or calls
- the dispatcher translates those calls into AIS
- the kernel applies AIS to state and can hand render primitives to rendering code

This separation is important. A parsing crate should not need to know how GPU readback works. A renderer should not need to understand HTML parsing. The kernel is the integration point where those separate concerns meet.

## Repository Principles

The docs and code should follow these principles:

- Document current behavior before future ambition.
- Say when a crate exists but is not an active workspace member.
- Prefer concrete command examples over broad claims.
- Treat benchmarks as measurements tied to hardware, profile, and command line.
- Keep compatibility layers honest about what they translate and what they implement.
- Keep demos useful, but do not let demo success imply full browser compatibility.

## Workspace Members

| Crate | Responsibility |
| --- | --- |
| `slate-ais` | Atomic instruction types, geometry primitives, render primitives, and state primitives. |
| `slate-dispatcher` | Stateless translation layer from `WebCall` into AIS streams. |
| `slate-state` | Deterministic state store and snapshots. |
| `slate-arena` | Per-page bump arena for short-lived allocations. |
| `slate-kernel` | Orchestration layer that owns dispatch, state, and arena lifecycle. |
| `slate-render` | Headless `wgpu` renderer for AIS render primitives. |
| `slate-script` | JavaScript runtime bridge that emits web calls. |
| `slate-network` | Streaming fetch layer with incremental parsing and origin policy hooks. |
| `slate-text` | Text shaping, glyph handling, and text layout primitives. |
| `slate-css` | CSS parsing, selector matching, cascade, and computed style logic. |
| `slate-html` | HTML parsing and tree construction. |
| `slate-events` | DOM event dispatch, listeners, and propagation model. |
| `slate-layout` | Layout engines for flexbox, block, inline, and grid flow. |
| `slate-dom` | DOM tree, mutation tracking, and query helpers. |
| `slate-rasterizer` | CPU rasterization utilities and display-list execution. |
| `slate-image` | Image decoding, loading, and cache helpers. |
| `slate-webapi` | Web API compatibility surface that translates API-like calls into the engine’s internal model. |
| `slate-benchmarks` | Benchmark harnesses for pipeline measurements. |
| `slate-window` | Standalone window entry point. |

These are the crates Cargo sees through the root workspace. When running `cargo check --workspace`, this set is the primary surface being verified.

## Experimental Crate Directories

The repository also contains these crate directories, but they are not currently listed in the root workspace members:

- `slate-storage`
- `slate-wasm`
- `slate-webgl`
- `slate-websocket`
- `slate-workers`

Treat these as staged or experimental until they are added to the root `Cargo.toml` workspace and verified through the normal build/test flow.

This distinction matters because source code can exist in the repository without being part of normal CI, build, or benchmark commands. A crate directory should be described as active only after it is listed in the workspace and participates in the normal commands documented here.

## Entry Points

The main runnable targets are:

- `slate-demo`
- `slate-phase2`
- `slate-pipeline`
- `slate-phase4-demo`

Example commands:

```bash
cargo run --release --bin slate-demo
cargo run --release --bin slate-phase2
cargo run --release --bin slate-pipeline
cargo run --release --bin slate-phase4-demo
```

### `slate-demo`

Runs a small hardcoded HTML-style sample through the early dispatcher/kernel path and prints the AIS stream and resulting state snapshot.

### `slate-phase2`

Exercises a higher-level path involving HTML parsing, CSS parsing, DOM construction, layout primitives, display-list construction, and CPU raster output.

### `slate-pipeline`

Runs a script-driven pipeline through the script runtime, kernel, and headless renderer. It may depend on local GPU availability for the render path.

### `slate-phase4-demo`

Exercises currently staged media and compatibility surfaces such as image operations, canvas-like primitives, form validation, and SVG-related code.

## Build and Verification

```bash
cargo build --workspace
cargo build --release
cargo check --workspace
cargo test --workspace
cargo bench -p slate-kernel
```

For targeted iteration:

```bash
cargo check -p slate-render
cargo test -p slate-css
cargo run -p slate-kernel --bin slate-demo --release
```

Recommended workflow before publishing changes:

1. Run `cargo check --workspace` for broad compile validation.
2. Run the relevant targeted test command for the crate you changed.
3. Run `python3 scripts/browser_engine_benchmark.py --profile debug --iterations 1 --warmups 0` if the change touches demos or pipeline behavior.
4. Run release benchmarks only when you need stable timing data.

## Current Scope

What is implemented in source today:

- AIS primitives and instruction streams
- dispatch and normalization logic
- deterministic state and page arena support
- headless rendering pipeline
- HTML, CSS, DOM, layout, events, text, image, and network modules
- Web API compatibility modules
- experimental WASM, WebGL, websocket, storage, and worker crate directories

What should be treated as evolving:

- Some crates expose partial or staged APIs.
- Several “compatibility” modules are wrappers or translators rather than complete browser-native implementations.

## What This Repository Does Not Claim

This repository does not currently claim:

- full HTML, CSS, DOM, JS, WebGL, WASM, WPT, or browser-standard compatibility
- a production security sandbox
- a complete multi-process browser architecture
- a finished browser UI
- stable public APIs for all crates
- performance parity with major browser engines

Those are useful long-term directions, but they should be tracked through code, tests, and benchmarks rather than asserted in documentation.

## Suggested Reading Order

1. [ARCHITECTURE.md](./ARCHITECTURE.md)
2. [FEATURES.md](./FEATURES.md)
3. [CAPABILITIES.md](./CAPABILITIES.md)
4. [BENCHMARKS.md](./BENCHMARKS.md)
5. [ROADMAP.md](./ROADMAP.md)
6. [crates/slate-webapi/README.md](./crates/slate-webapi/README.md)

## Detailed Repository Tour

The repository is easiest to understand as several rings around the AIS core.

The innermost ring is the core execution model:

- `crates/slate-ais` defines the instruction vocabulary.
- `crates/slate-dispatcher` turns web-facing calls into instruction streams.
- `crates/slate-state` applies state-oriented instructions into a deterministic store.
- `crates/slate-arena` handles page-scoped temporary allocation.
- `crates/slate-kernel` ties those pieces together.

The next ring is document and visual processing:

- `crates/slate-html` parses HTML-like input.
- `crates/slate-css` parses and resolves CSS-like styling data.
- `crates/slate-dom` stores and mutates document-like tree structures.
- `crates/slate-layout` computes layout primitives.
- `crates/slate-rasterizer` handles CPU display-list output.
- `crates/slate-render` handles headless GPU-oriented rendering.

The outer ring is integration and compatibility:

- `crates/slate-script` bridges JavaScript execution into owned web calls.
- `crates/slate-network` handles fetch-oriented input and incremental parsing hooks.
- `crates/slate-webapi` exposes selected compatibility-oriented helpers.
- `crates/slate-window` is the current standalone window entry point.

Finally, the repository contains staged directories that are intentionally documented separately:

- `crates/slate-storage`
- `crates/slate-wasm`
- `crates/slate-webgl`
- `crates/slate-websocket`
- `crates/slate-workers`

These staged directories can become active workspace members later, but until then they should not be described as part of the normal `cargo check --workspace` surface.

## Contributor Mental Model

When changing this repository, first identify which layer you are touching.

| If you change | You are probably affecting | Recommended verification |
| --- | --- | --- |
| AIS primitives | dispatch, state, renderer, demos | `cargo check --workspace`, relevant demo, benchmark harness |
| dispatcher normalization | kernel input behavior | dispatcher tests, `slate-demo`, Python benchmark |
| state application | snapshots and replay | state tests, `slate-demo`, pipeline run |
| renderer internals | artifact output | `slate-pipeline`, artifact checks |
| HTML/CSS/DOM/layout code | document pipeline behavior | `slate-phase2`, targeted crate tests |
| script bridge | JS-to-kernel path | `slate-pipeline`, script-specific tests |
| docs only | contributor understanding | link scan, command accuracy check |

This style of review keeps changes from being validated only at the wrong layer. For example, a renderer change should be checked with an artifact-producing path, not only a compile command.

## Practical Development Workflow

A conservative local workflow looks like this:

```bash
cargo check --workspace
cargo test --workspace
python3 scripts/browser_engine_benchmark.py --profile debug --iterations 1 --warmups 0
```

For tighter iteration, run only the crate you touched:

```bash
cargo check -p slate-dispatcher
cargo test -p slate-dispatcher
cargo run -p slate-kernel --bin slate-demo
```

For timing-sensitive work, use release mode and multiple iterations:

```bash
python3 scripts/browser_engine_benchmark.py --profile release --iterations 10 --warmups 2
```

Do not compare debug timings to release timings. Debug runs are good for correctness and smoke validation; release runs are the only sensible timing baseline.

## Documentation Standards

All Markdown files should follow the same truthfulness rules:

- prefer “the crate exists” over “the feature is complete”
- prefer “the demo exercises this path” over “the engine supports this standard”
- prefer exact command lines over broad workflow claims
- note when a directory is not in the active workspace
- remove stale phase language when it stops matching the code
- avoid comparing Slate to major browser engines without measured data

When adding a new Markdown section, ask whether the statement can be verified by one of these:

- root `Cargo.toml`
- `cargo metadata`
- source file exports
- demo output
- benchmark report
- a test or compile command

## Release Readiness Checklist

Before pushing a public branch or preparing a release-like snapshot:

- confirm `repository` metadata in `Cargo.toml` points at the real GitHub repository
- run `cargo check --workspace`
- run targeted tests for touched crates
- run the Python benchmark harness at least once in debug mode
- confirm generated artifacts are ignored by `.gitignore`
- review README and benchmark docs for stale command lines
- confirm experimental directories are not described as active workspace members

## Glossary

| Term | Meaning in this repository |
| --- | --- |
| AIS | Atomic Instruction Set, the internal instruction vocabulary. |
| WebCall | The normalized web-facing call representation consumed by the dispatcher. |
| Kernel | The integration point that dispatches calls, applies state, and exposes snapshots. |
| Headless renderer | Rendering code that writes to an offscreen target instead of a browser window. |
| Active workspace member | A crate listed in the root `Cargo.toml` workspace members. |
| Experimental directory | A crate-like directory present in source but not included in root workspace members. |
| Demo | Runnable binary that exercises a path through the engine; not a compliance suite. |
| Benchmark harness | Tooling that measures real execution paths and records structured output. |

## License

Apache-2.0 OR MIT, at your option.
