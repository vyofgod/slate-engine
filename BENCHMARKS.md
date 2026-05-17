# Slate Engine Benchmarks

This document describes the benchmark entry points that exist in the repository and the rough intent of each one. For the system view, see [README.md](./README.md) and [ARCHITECTURE.md](./ARCHITECTURE.md).

## Benchmark Philosophy

Benchmarks in this repository should answer concrete engineering questions:

- Does the dispatcher emit a stable instruction stream?
- How long do demo pipelines take under a given profile?
- Did a render path produce an artifact with expected dimensions?
- Did a change affect state, layout, or render output?
- Are timing numbers being collected under repeatable command-line conditions?

Benchmark results should be treated as local measurements. Hardware, GPU availability, Rust profile, system load, and dependency versions can all change results.

## Run Commands

```bash
python3 scripts/browser_engine_benchmark.py --iterations 5 --warmups 1
python3 scripts/browser_engine_benchmark.py --iterations 3 --criterion-smoke
./scripts/benchmark.sh
cargo bench -p slate-kernel
cargo bench -p slate-css
cargo bench -p slate-layout
cargo bench -p slate-benchmarks --bench full_pipeline
```

## Browser Engine Harness

`scripts/browser_engine_benchmark.py` is the preferred high-level benchmark runner for engine behavior. It is a Python harness, not a shell script. It builds the real Rust binaries, runs the browser-engine demo paths, parses their output, validates generated render artifacts, and writes reports to `target/slate-browser-benchmark/`.

Measured scenarios:

- `ais_dispatch_demo`: HTML snippet to WebCall, AIS, and state snapshot.
- `html_css_layout_raster`: HTML, CSS, DOM, layout, display list, and CPU raster path.
- `script_kernel_gpu_pipeline`: script runtime, kernel dispatch, and headless GPU render path.
- `media_forms_svg_surface`: image, canvas, form validation, and SVG compatibility path.

Generated reports:

- `target/slate-browser-benchmark/latest.json`
- `target/slate-browser-benchmark/latest.md`

Recommended quick validation:

```bash
python3 scripts/browser_engine_benchmark.py --profile debug --iterations 1 --warmups 0
```

Recommended timing run:

```bash
python3 scripts/browser_engine_benchmark.py --profile release --iterations 10 --warmups 2
```

Use debug runs to verify functionality quickly. Use release runs for timing comparisons.

## Benchmark Areas

### Dispatcher

The dispatcher benchmark focuses on translation overhead in `slate-kernel` and `slate-dispatcher`.

Typical measurements:

- element creation translation
- inline style translation
- append-child translation

Dispatcher benchmarks are useful for catching instruction-count regressions. They are not enough to prove real page performance by themselves.

### CSS Selector Matching

The CSS benchmark area focuses on selector matching and cascade-related work in `slate-css`.

Typical measurements:

- simple selectors
- compound selectors
- specificity calculation
- cascade resolution

### Layout

The layout benchmark area focuses on the engines in `slate-layout`.

Typical measurements:

- flexbox row/column layouts
- wrapping behavior
- alignment modes
- nested layout structures

### Full Pipeline

The full pipeline benchmark measures end-to-end work across parsing, dispatch, state application, and rendering.

It is the most useful benchmark when validating changes that affect the entire engine flow.

Full pipeline results should include:

- command used
- profile used
- number of warmups and iterations
- generated report path
- whether render artifacts were produced
- whether GPU rendering was skipped because no adapter was available

## Legacy Shell Scripts

`scripts/benchmark.sh`, `scripts/compare_benchmarks.sh`, and `scripts/performance_report.sh` are older helper scripts. Prefer `scripts/browser_engine_benchmark.py` for current high-level engine benchmarking because it validates artifacts and writes structured reports.

The older shell scripts can still be useful for local quick checks, but their generated prose may contain older framing. Treat the Python harness as the current source of structured benchmark reporting.

## Notes on Targets

The numeric performance targets in older documents should be treated as goals, not guarantees. Use them as directional markers and confirm actual results with the current code and hardware.

When adding a new target, include:

- what command measures it
- what profile it uses
- what hardware or environment it was observed on
- what output proves success

## Related Documents

- [FEATURES.md](./FEATURES.md)
- [ROADMAP.md](./ROADMAP.md)

## Python Harness Report Shape

The Python harness writes both JSON and Markdown. The JSON file is the better source for automation, while the Markdown file is easier to read in reviews.

Important JSON fields:

| Field | Meaning |
| --- | --- |
| `schema_version` | Report schema version for future compatibility. |
| `generated_at_unix` | Generation timestamp. |
| `profile` | Cargo profile used by the run. |
| `iterations` | Number of measured iterations. |
| `warmups` | Number of warmup runs before measurement. |
| `build_ms` | Time spent building binaries, if build was not skipped. |
| `platform` | Basic OS, machine, and Python information. |
| `scenarios` | Per-scenario timing, metrics, artifacts, and excerpts. |

Each scenario records:

- scenario name
- binary name
- description
- timing summary
- extracted engine metrics
- per-iteration stdout/stderr excerpt
- generated artifact metadata when applicable

Artifact metadata can include:

- existence
- byte size
- SHA-256 hash
- PPM width
- PPM height
- PPM max color value

## Benchmark Interpretation Guide

Use benchmark output carefully.

If `script_kernel_gpu_pipeline` says `gpu_skipped: true`, the run did not measure the GPU render path. That is still useful as a smoke test for earlier stages, but it should not be used as a GPU performance number.

If a PPM artifact exists with expected dimensions, that proves the demo wrote an image-shaped output. It does not prove that the visual rendering is correct in a browser-compliance sense. Visual correctness needs snapshot testing, pixel comparisons, or manual review.

If one iteration is much slower than others, check for:

- first-run build effects
- shader or GPU initialization
- OS scheduling noise
- debug vs release profile mismatch
- background processes
- artifact directory creation

## Adding New Benchmarks

When adding a benchmark, decide which level it belongs to:

| Level | Use when | Tool |
| --- | --- | --- |
| Micro | Measuring a small function or algorithm | Criterion |
| Crate-level | Measuring a subsystem in isolation | Criterion or targeted test harness |
| Demo pipeline | Measuring real engine demo behavior | Python harness |
| Artifact validation | Confirming generated files exist and are shaped correctly | Python harness |

For a new Python harness scenario:

1. Add a `Scenario` entry in `scripts/browser_engine_benchmark.py`.
2. Point it at a real binary.
3. Add a parser that extracts meaningful stdout/stderr metrics.
4. List expected artifacts if the scenario writes files.
5. Run one debug iteration.
6. Document the scenario in this file.

For a new Criterion benchmark:

1. Add a bench target to the crate `Cargo.toml`.
2. Keep sample input deterministic.
3. Avoid measuring setup unless setup is part of the behavior being studied.
4. Document the command in this file.
5. Prefer names that describe the subsystem and operation.
