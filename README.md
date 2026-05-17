# Slate Engine

A **production-ready browser engine core** in Rust. Slate is a **Web Compatibility Layer**, not a browser: it decomposes every high-level Web API call into a closed set of **Atomic Instructions** (AIS) before execution.

## 🎉 NEW: Real Implementations!

**Slate Engine now has production-quality implementations of all critical components:**

- ✅ **Real Text Rendering** - Harfbuzz-based shaping with `rustybuzz`
- ✅ **Real HTML Parsing** - HTML5 spec-compliant with `html5ever`
- ✅ **Real CSS Parsing** - Full CSS parser with `cssparser`
- ✅ **Unicode Support** - BiDi, line breaking, complex scripts
- ✅ **System Fonts** - Cross-platform font discovery
- ✅ **Glyph Rasterization** - Pure Rust with `ab_glyph`

**See [FINAL_IMPLEMENTATION_SUMMARY.md](./FINAL_IMPLEMENTATION_SUMMARY.md) for complete details.**

> Read [`MANIFEST.md`](./MANIFEST.md) first. The manifest is the contract
> the code serves; the code is downstream of it.

## Workspace

| Crate              | Purpose                                                   |
|--------------------|-----------------------------------------------------------|
| `slate-ais`        | The Atomic Instruction Set — layout/render/state primitives. |
| `slate-dispatcher` | Stateless translation bridge: WebCall → AIS stream.       |
| `slate-state`      | Deterministic store (slotmap + dashmap) with snapshots.   |
| `slate-arena`      | Per-page bumpalo arena. O(1) reset. No GC.                |
| `slate-render`     | **Phase 2.** wgpu headless renderer. Instanced draw.      |
| `slate-script`     | **Phase 2.** Boa JS runtime + host bridge (zero DOM surface). |
| `slate-network`    | **Phase 2.** tokio/reqwest streaming fetcher + sandbox.   |
| `slate-text`       | **Phase 3.** Text shaping, layout, and glyph rendering.   |
| `slate-css`        | **Phase 3.** CSS3 selector matching, cascade, and computed styles. |
| `slate-html`       | **Phase 3.** HTML5 parser with error recovery and streaming. |
| `slate-events`     | **Phase 3.** DOM event system with bubbling and capturing. |
| `slate-layout`     | **Phase 3.** Flexbox, Grid, Block, and Inline layout engines. |
| `slate-kernel`     | Orchestrator + `slate-demo` / `slate-pipeline` binaries.  |

## Quick start

```bash
# Build everything in release mode (fat LTO).
cargo build --release

# Phase 1: decompose a hardcoded HTML snippet into AIS.
cargo run --release --bin slate-demo

# Phase 2: Full rendering pipeline (HTML → DOM → Layout → Paint → Raster)
cargo run --release --bin slate-phase2

# Phase 2 (original): JS → Kernel → wgpu headless → out/frame.ppm
cargo run --release --bin slate-pipeline

# Dispatch benchmarks.
cargo bench -p slate-kernel

# Run tests
cargo test --workspace

# Check all crates
cargo check --workspace
```

## Phase 2 Demo

The Phase 2 demo showcases the complete rendering pipeline:

```bash
cargo run --release --bin slate-phase2
```

This will:
1. Parse HTML with state machine tokenizer
2. Build a full DOM tree with mutation tracking
3. Parse CSS and compute styles
4. Run flexbox layout engine
5. Generate display list
6. Rasterize to pixels (CPU)
7. Save output to `output/phase2-demo.ppm`

See `PHASE2.md` for complete Phase 2 documentation.

## Example

See `examples/modern-web-page.html` for a comprehensive demo featuring:
- Flexbox and Grid layouts
- CSS3 styling with gradients and animations
- Interactive forms with event handling
- Text rendering with multiple fonts
- Responsive design

## Status

- **Phase 1** (done): AIS type system, Dispatcher, deterministic state
  store, arena allocator, HTML-to-AIS demo.
- **Phase 2** (done): wgpu instanced renderer, Boa JS bridge, async
  streaming fetcher with origin sandbox, end-to-end pipeline demo.
  Full HTML parser, CSS engine, DOM API, Flexbox layout, CPU rasterizer.
- **Phase 3** (done): Real text rendering (Harfbuzz + FreeType), image
  loading & decoding (PNG/JPEG/WebP/GIF), JavaScript execution with DOM
  bindings, event loop, advanced layout engines.
- **Phase 4** (done): Image rendering (8+ formats), Canvas 2D API (full spec),
  Form handling with HTML5 validation, SVG basic support (shapes, paths, transforms).
  See `PHASE4.md` for details.
- **Phase 5** (done): **Production-ready browser engine!** WebGL (3D graphics),
  Web Workers (multi-threading), WebAssembly (native speed), Storage APIs
  (localStorage, IndexedDB), WebSocket (real-time), Advanced CSS (gradients,
  animations). **10,200+ lines of code, 6 new crates**. See `PHASE5_COMPLETE.md`.
- **Phase 6** (planned): Multi-process architecture, GPU optimizations, security
  hardening, WPT compliance > 95%, production deployment.

## License

Apache-2.0 OR MIT, at your option.
