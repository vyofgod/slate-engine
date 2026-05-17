# Slate Web API Compatibility Layer

This crate documents the compatibility surface around web-facing APIs. It should be read together with the root [README](../../README.md) and [ARCHITECTURE.md](../../ARCHITECTURE.md).

## Purpose

`slate-webapi` is a translation-oriented crate. The current source emphasizes converting web-facing actions into the engine’s internal model rather than re-creating a full browser implementation.

The key distinction is scope: this crate is about compatibility surfaces and translation behavior. It should not be documented as if it alone provides a complete DOM, event loop, network stack, renderer, or browser shell.

## Current Modules

The public module list in source includes:

- `translator`
- `canvas`
- `forms`
- `svg`

Some older module names are commented out in source and should not be treated as active public APIs.

When adding or re-enabling a module, update this list only after the module is exported from `src/lib.rs` and covered by at least a basic compile or integration check.

## Translation Model

The general pattern is:

```text
web-facing action
  -> translator
  -> internal engine representation
  -> dispatcher / kernel
```

This crate is not the same thing as the kernel, dispatcher, or renderer. It is a compatibility layer that feeds them.

## Integration Expectations

A healthy `slate-webapi` integration should make these questions easy to answer:

- What web-facing operation entered the crate?
- What internal call or data structure was produced?
- Can the dispatcher consume that output?
- Can the kernel apply the resulting instructions?
- Is unsupported behavior reported clearly?

## Practical Notes

- Treat API completeness claims cautiously and verify against the code.
- Prefer the root documentation for system-wide architecture.
- Use this crate’s source when you need the exact public surface.

## Documentation Rule

Do not document a Web API as complete simply because a module name exists. Document exact behavior: parser, translator, helper type, demo path, or test coverage.

## Related Documents

- [Root README](../../README.md)
- [Architecture](../../ARCHITECTURE.md)
- [Capabilities](../../CAPABILITIES.md)

## Module Notes

### `translator`

The translator module is the core compatibility-oriented adapter. Documentation should describe what it accepts and emits rather than implying a complete browser API surface.

### `canvas`

Canvas-related helpers should be documented as helper or compatibility code unless a full drawing API is tested through the engine pipeline.

### `forms`

Forms support should be described by exact validation or helper behavior. Do not infer form submission, navigation, browser UI, or accessibility behavior from validation helpers alone.

### `svg`

SVG support should be described by exact parsed or rendered primitives. Avoid saying “SVG support” without naming which shapes, attributes, transforms, or output paths are covered.

## Verification Commands

Useful commands when changing this crate:

```bash
cargo check -p slate-webapi
cargo test -p slate-webapi
cargo run -p slate-kernel --bin slate-phase4-demo
python3 scripts/browser_engine_benchmark.py --profile debug --iterations 1 --warmups 0
```

Use the crate-level commands for compile/test confidence and the demo/harness commands for integration confidence.
