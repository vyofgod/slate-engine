# Slate Engine Roadmap

This roadmap documents planned or partially staged work. It should be read alongside [README.md](./README.md), [ARCHITECTURE.md](./ARCHITECTURE.md), and [FEATURES.md](./FEATURES.md).

## Guiding Goal

Continue turning the current workspace into a more complete browser-engine style system while preserving the explicit dispatch/state/render architecture.

## Roadmap Rules

Roadmap items should stay grounded. A future item belongs here when it is directionally important but not yet reliable enough to describe as a current capability.

When work graduates from roadmap to current state:

- update the code or workspace membership first
- add tests or benchmark coverage where appropriate
- move the claim into [FEATURES.md](./FEATURES.md) or [CAPABILITIES.md](./CAPABILITIES.md)
- keep this file focused on what remains ahead

## Near-Term Priorities

### 1. Documentation and Truthfulness

- Keep the public docs aligned with the codebase.
- Remove stale claims as the source evolves.
- Link related docs instead of duplicating content.

Concrete tasks:

- keep workspace member lists synced with root `Cargo.toml`
- remove old “phase complete” language unless backed by tests
- keep benchmark commands executable
- describe experimental crate directories separately from active crates

### 2. Runtime Completeness

- Fill in missing pieces in the script/runtime compatibility surfaces.
- Tighten network and storage behavior where currently partial.
- Improve error handling and surface consistency across crates.

Concrete tasks:

- clarify which Web API modules are active
- route compatibility outputs through dispatcher/kernel paths
- add focused tests for translator behavior
- document unsupported inputs explicitly

### 3. Rendering and Layout

- Expand coverage in layout engines.
- Continue improving the headless rendering path.
- Reduce gaps between the demo pipelines and the crate-level APIs.

Concrete tasks:

- keep PPM or equivalent artifact generation stable
- add fixture-style layout tests
- measure layout output counts in benchmark reports
- separate CPU raster and GPU render paths in docs and reports

### 4. Text and Media

- Continue text shaping and font handling work.
- Improve image support and decode coverage.
- Verify how these subsystems integrate with layout and rendering.

Concrete tasks:

- add format-specific image tests before broad image-support claims
- benchmark representative text layout inputs
- document font and glyph behavior in a source-aligned way

## Longer-Term Work

- Security hardening
- More complete multi-process separation
- Deeper platform compatibility
- Expanded benchmark coverage
- Better developer tooling around demos and pipeline inspection

## Open Questions

These are useful design questions for future work:

- Which experimental crate directories should become active workspace members first?
- What is the minimum compatibility target for each web-facing surface?
- Which artifacts should be stable enough for regression testing?
- Should benchmark reports include memory measurements in addition to timing?
- Which demos should become formal integration tests?

## Working Rule

When a roadmap item becomes implemented, it should move into [FEATURES.md](./FEATURES.md) and be reflected in [CAPABILITIES.md](./CAPABILITIES.md). The roadmap is for future work, not a second copy of current state.

## Milestone Candidates

### Milestone A: Documentation Baseline

Goal: make the repository understandable and honest from a fresh clone.

Exit criteria:

- README lists active workspace crates accurately.
- Experimental directories are separated from active capabilities.
- Benchmark commands are executable.
- Web API docs do not claim full browser parity.
- Roadmap items are not duplicated as current features.

### Milestone B: Workspace Hygiene

Goal: make active workspace membership intentional.

Exit criteria:

- every active crate compiles through `cargo check --workspace`
- crate descriptions and docs match root workspace membership
- experimental directories either become active or remain clearly staged
- dependency versions are reviewed before promoting staged crates

### Milestone C: Pipeline Confidence

Goal: make the core demos reliable enough to catch regressions.

Exit criteria:

- `slate-demo` emits stable instruction metrics
- `slate-phase2` writes a validated raster artifact
- `slate-pipeline` reports GPU skip status clearly
- `slate-phase4-demo` reports compatibility-surface metrics
- Python benchmark reports are generated consistently

### Milestone D: Feature Hardening

Goal: move from “source exists” to “behavior is reliable.”

Exit criteria:

- targeted tests exist for important parser, dispatcher, state, and layout behavior
- render artifacts have regression checks
- compatibility surfaces document unsupported behavior
- roadmap items are promoted only after validation

## Risk Register

| Risk | Why it matters | Mitigation |
| --- | --- | --- |
| Docs overclaim standards support | Misleads users and contributors | Use source-aligned language and capability labels. |
| Experimental crates appear active | Confuses build expectations | Keep active workspace and experimental sections separate. |
| Benchmarks become stale | Performance claims become untrustworthy | Prefer Python harness and document exact commands. |
| Demos hide subsystem failures | A successful demo can mask partial behavior | Add targeted tests and extract stage metrics. |
| Rendering depends on local GPU | Results differ across machines | Record GPU skip status and artifact metadata. |
| AIS grows without review | Internal contract becomes harder to reason about | Require design justification and validation plan. |

## Decision Log Template

Use this shape when recording future architectural decisions:

```md
## Decision: Short Name

Context:
- What problem forced the decision?

Decision:
- What was chosen?

Consequences:
- What becomes easier?
- What becomes harder?

Validation:
- What command, test, or benchmark proves the decision works?
```
