# Slate Engine

A minimalist browser engine core in Rust. Slate is a **Web Compatibility
Layer**, not a browser: it decomposes every high-level Web API call into
a closed set of **Atomic Instructions** (AIS) before execution.

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
| `slate-kernel`     | Orchestrator + `slate-demo` / `slate-pipeline` binaries.  |

## Quick start

```bash
# Build everything in release mode (fat LTO).
cargo build --release

# Phase 1: decompose a hardcoded HTML snippet into AIS.
cargo run --release --bin slate-demo

# Phase 2: JS → Kernel → wgpu headless → out/frame.ppm
cargo run --release --bin slate-pipeline

# Dispatch benchmarks.
cargo bench -p slate-kernel
```

## Status

- **Phase 1** (done): AIS type system, Dispatcher, deterministic state
  store, arena allocator, HTML-to-AIS demo.
- **Phase 2** (done): wgpu instanced renderer, Boa JS bridge, async
  streaming fetcher with origin sandbox, end-to-end pipeline demo.
- **Phase 3** (planned): HTTP/3 transport, layout pass beyond raw
  positioning, text/glyph AIS primitives, Wasm SIMD fast path.

## License

Apache-2.0 OR MIT, at your option.
