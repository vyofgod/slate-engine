# Slate Engine Architecture

## Overview

Slate is a production-ready browser engine built in Rust with a revolutionary architecture that eliminates the "interpretation tax" found in mainstream engines (Blink, WebKit, Gecko).

## Core Philosophy

**API Elimination via Atomic Reduction**: Every Web API call reduces to 200-500 atomic primitives across three domains (Layout, Render, State) before execution. No interpretation, only translation.

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│  Web Platform (HTML5, CSS3, JavaScript, Web APIs)           │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  High-Level Engines                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ HTML5 Parser │  │  CSS Engine  │  │ Event System │      │
│  │  (slate-html)│  │ (slate-css)  │  │(slate-events)│      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Text Engine  │  │Layout Engine │  │  JS Runtime  │      │
│  │ (slate-text) │  │(slate-layout)│  │(slate-script)│      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Dispatcher (slate-dispatcher)                               │
│  Stateless translation: WebCall → AIS stream                 │
│  (a) Intercept  (b) Normalize  (c) Decompose  (d) Inline    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼ AtomicInstruction stream
┌─────────────────────────────────────────────────────────────┐
│  Kernel + State Store (slate-kernel + slate-state)           │
│  Deterministic execution with snapshots                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  GPU Pipeline (slate-render)                                 │
│  Vulkan / Metal / WebGPU - Zero-copy instanced rendering     │
└─────────────────────────────────────────────────────────────┘
```

## Crate Breakdown

### Core Infrastructure

#### `slate-ais` - Atomic Instruction Set
The machine code of Slate. Three domains:
- **Layout**: Pure geometry (SetPosition, SetSize, SetClip, FlexBasis)
- **Render**: GPU commands (FillRect, StrokeRect, DrawText, FillPath)
- **State**: Functional state deltas (deterministic, replayable)

Target: 200-500 total primitives. Growth = design failure.

#### `slate-dispatcher` - Translation Bridge
Stateless Wine-like syscall translator:
- Input: `WebCall<'a>` (borrowed from JS/HTML)
- Output: `Stream` of `AtomicInstruction`
- Pipeline: Normalize → Decompose → Inline
- Zero state, pure function, O(n) single-pass

#### `slate-state` - Deterministic Store
- `slotmap` + `dashmap` for concurrent access
- Immutable snapshots for time-travel debugging
- `(snapshot, inputs) → snapshot'` is pure
- No GC, no wall clock, no allocator leaks

#### `slate-arena` - Per-Page Allocator
- `bumpalo` arena per page
- O(1) reset on navigation
- No garbage collection overhead

#### `slate-kernel` - Orchestrator
- Consumes AIS streams
- Manages state store
- Coordinates rendering
- Hosts demo binaries

### Phase 2: Execution & Visualization

#### `slate-render` - GPU Renderer
- wgpu-based (Vulkan/Metal/WebGPU)
- Instanced drawing (single draw call per frame)
- No CPU paint phase
- Zero-copy to GPU (`repr(C)` primitives)
- Offscreen Rgba8UnormSrgb target

#### `slate-script` - JavaScript Runtime
- Boa `Context` on isolated thread
- Three host functions only:
  - `__slate_create_element`
  - `__slate_append_child`
  - `__slate_set_style`
- Pushes `OwnedWebCall` to buffer
- Kernel drains buffer each frame
- No DOM access, no wall time, deterministic

#### `slate-network` - Async Fetcher
- tokio + reqwest streaming
- `IncrementalParser` for chunked HTML
- `OriginPolicy` sandbox
- First AIS before full download

### Phase 3: Modern Browser Features

#### `slate-text` - Text Rendering
**Status**: Foundation complete, harfbuzz integration pending

Features:
- Font loading and caching (`FontCache`)
- Text shaping with BiDi support (`TextShaper`)
- Line breaking and word wrapping (`LineBreaker`)
- Glyph positioning (`GlyphRun`, `PositionedGlyph`)
- Font metrics and typography

Architecture:
```rust
Text → TextShaper → GlyphRun → LineBreaker → TextLayout → RenderPrimitive::DrawText
```

#### `slate-css` - CSS3 Engine
**Status**: Core complete, parser integration pending

Features:
- Full CSS3 selector matching
- Specificity calculation
- Cascade resolution
- Computed styles with inheritance
- Property parsing (100+ properties)

Components:
- `Selector`: Universal, Type, Class, ID, Attribute, Pseudo-class, Combinator
- `Specificity`: (inline, ids, classes, elements) ordering
- `CascadeEngine`: Matches selectors, resolves cascade, computes styles
- `SelectorMatcher`: Fast node-to-selector matching

#### `slate-html` - HTML5 Parser
**Status**: Foundation complete, html5ever integration pending

Features:
- HTML5 spec-compliant parsing
- Error recovery and quirks mode
- DOCTYPE handling
- Streaming incremental parsing
- DOM tree construction

Output:
- `DomTree`: Node hierarchy
- `Vec<OwnedWebCall>`: AIS-ready operations

#### `slate-events` - Event System
**Status**: Complete

Features:
- Full DOM event model
- Bubbling and capturing phases
- `preventDefault()` and `stopPropagation()`
- Event types: Mouse, Keyboard, Touch, Pointer, Wheel, Focus, Drag, Scroll
- Event delegation and listener management

Architecture:
```rust
Event → Capturing Phase → At Target → Bubbling Phase
```

#### `slate-layout` - Layout Engines
**Status**: Core algorithms complete

Engines:
1. **FlexLayout**: CSS Flexbox
   - Flex direction, wrap, justify-content, align-items
   - Flex grow/shrink/basis
   - Gap support
   
2. **GridLayout**: CSS Grid
   - Template columns/rows
   - Track sizing (fr, fixed, auto, minmax)
   - Grid item placement
   - Gap support

3. **BlockLayout**: Normal flow
   - Vertical stacking
   - Margin collapse
   - Block formatting context

4. **InlineLayout**: Text flow
   - Horizontal flow with wrapping
   - Baseline alignment
   - Line height

## Key Invariants (Non-Negotiable)

1. **Closure under decomposition**: If a Web call cannot reduce to AIS, it's not supported. No fallback paths.

2. **Determinism**: `(snapshot, input_sequence) → snapshot'` is pure. No wall clock, no thread scheduling, no allocator addresses in observable state.

3. **Single-pass dispatch**: Dispatcher produces AIS in O(n). No fix-point, no retry, no layout thrashing.

4. **Zero-copy to GPU**: Render primitives are `repr(C)` and sized for direct upload. No intermediate CPU paint buffer.

5. **No GC**: Per-page state in `bumpalo` arena. O(1) reset on navigation. `unsafe` only with benchmark justification.

## Performance Characteristics

### Memory
- Per-page arena allocation: O(1) reset
- No garbage collection pauses
- Shared font/image caches
- Target: <50MB per average page

### Speed
- Single-pass dispatch: O(n) over input
- Parallel layout: SIMD-friendly primitives
- Instanced GPU rendering: 1 draw call/frame
- Target: <500ms first meaningful paint

### Scalability
- Embarrassingly parallel layout (pure geometry)
- Lock-free state store (`dashmap`)
- Multi-threaded dispatch (Phase 4)
- Process-per-tab isolation (Phase 4)

## Security Model

### Current (Phase 3)
- Origin-based network sandbox
- No eval, no Function constructor
- Isolated JS runtime (no DOM access)
- Deterministic execution (no timing attacks)

### Planned (Phase 4)
- Multi-process architecture
- Renderer process isolation
- GPU process separation
- Seccomp filters (Linux)
- Sandbox profiles (macOS)
- AppContainer (Windows)

## Testing Strategy

### Unit Tests
- Per-crate test suites
- Property-based testing for layout
- Snapshot testing for rendering

### Integration Tests
- End-to-end pipeline tests
- Real HTML/CSS rendering
- JavaScript integration

### Compliance (Phase 4)
- Web Platform Tests (WPT)
- Acid3 test
- HTML5 test suite
- CSS test suite
- JavaScript test262

Target: >95% WPT pass rate

## Build System

### Profiles
- **Release**: Fat LTO, single codegen unit, max optimization
- **Bench**: Thin LTO, debug symbols for flamegraphs
- **Dev**: Fast iteration, optimized dependencies

### Dependencies
- Minimal external deps
- No proc-macro heavy crates in hot paths
- Prefer `no_std` where possible

## Future Roadmap

### Phase 4: Production Ready (6-12 months)
- [ ] Multi-process architecture
- [ ] WebGL/Canvas 2D
- [ ] Video/Audio elements
- [ ] WebAssembly integration
- [ ] Service Workers
- [ ] IndexedDB
- [ ] DevTools protocol

### Phase 5: Performance Leadership (12-18 months)
- [ ] SIMD layout fast path
- [ ] JIT compilation (Cranelift)
- [ ] HTTP/3 with QUIC
- [ ] Advanced GPU optimizations
- [ ] Memory compression
- [ ] Faster than Chromium

### Phase 6: Ecosystem (18-24 months)
- [ ] Embedder API (C FFI)
- [ ] Language bindings (Python, Node.js)
- [ ] WebView component
- [ ] Community contributions
- [ ] Real-world adoption

## Success Metrics

- **Performance**: Speedometer 3.0 > 400
- **Compliance**: WPT pass rate > 95%
- **Memory**: Average page < 50MB
- **Speed**: First meaningful paint < 500ms
- **Security**: Zero critical vulnerabilities
- **Adoption**: 1000+ stars, 100+ contributors

## Contributing

Before adding code, ask: *Does this re-introduce abstraction bloat?*

Rejected reasons:
- "It's more ergonomic"
- "It matches the spec more literally"
- "It's how Chromium does it"

Accepted reasons:
- Shrinks the primitive set
- Tightens determinism
- Removes a CPU-bound phase
- Improves security
- Measurable performance gain

## License

Apache-2.0 OR MIT, at your option.

---

**This document is the contract. The code is downstream of it.**
