#!/usr/bin/env python3
"""Slate browser-engine benchmark harness.

This is intentionally not a shell wrapper. It builds the real Rust engine
binaries, executes browser-engine style workloads, validates their artifacts,
extracts stage metrics from stdout, and writes machine-readable reports.
"""

from __future__ import annotations

import argparse
import dataclasses
import hashlib
import json
import os
import platform
import re
import statistics
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_REPORT_DIR = ROOT / "target" / "slate-browser-benchmark"


@dataclasses.dataclass(frozen=True)
class Scenario:
    name: str
    binary: str
    description: str
    artifacts: tuple[Path, ...]
    parser: Callable[[str], dict[str, Any]]
    allow_gpu_skip: bool = False


@dataclasses.dataclass
class IterationResult:
    iteration: int
    elapsed_ms: float
    exit_code: int
    stdout_bytes: int
    stderr_bytes: int
    metrics: dict[str, Any]
    artifacts: dict[str, Any]
    stdout_excerpt: str
    stderr_excerpt: str


def run_command(
    args: list[str],
    cwd: Path,
    timeout: float,
    env: dict[str, str] | None = None,
) -> tuple[int, float, str, str]:
    start = time.perf_counter_ns()
    proc = subprocess.run(
        args,
        cwd=str(cwd),
        env=env,
        text=True,
        capture_output=True,
        timeout=timeout,
        check=False,
    )
    elapsed_ms = (time.perf_counter_ns() - start) / 1_000_000.0
    return proc.returncode, elapsed_ms, proc.stdout, proc.stderr


def cargo_metadata(timeout: float) -> dict[str, Any]:
    code, _, stdout, stderr = run_command(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        ROOT,
        timeout,
    )
    if code != 0:
        raise RuntimeError(f"cargo metadata failed:\n{stderr}")
    return json.loads(stdout)


def build_engine(profile: str, timeout: float) -> float:
    cmd = ["cargo", "build", "-p", "slate-kernel", "--bins"]
    if profile == "release":
        cmd.append("--release")
    code, elapsed_ms, stdout, stderr = run_command(cmd, ROOT, timeout)
    if code != 0:
        raise RuntimeError(
            "cargo build failed\n"
            f"command: {' '.join(cmd)}\n"
            f"stdout:\n{stdout}\n"
            f"stderr:\n{stderr}"
        )
    return elapsed_ms


def executable_path(target_dir: Path, profile: str, binary: str) -> Path:
    suffix = ".exe" if platform.system().lower().startswith("win") else ""
    return target_dir / profile / f"{binary}{suffix}"


def parse_int(pattern: str, text: str, default: int | None = None) -> int | None:
    match = re.search(pattern, text, re.MULTILINE)
    if not match:
        return default
    return int(match.group(1))


def parse_demo(stdout: str) -> dict[str, Any]:
    return {
        "layout_instructions": len(re.findall(r"^\s+\[LAY\]", stdout, re.MULTILINE)),
        "render_instructions": len(re.findall(r"^\s+\[REN\]", stdout, re.MULTILINE)),
        "state_instructions": len(re.findall(r"^\s+\[STA\]", stdout, re.MULTILINE)),
        "snapshot_version": parse_int(r"version = (\d+)", stdout, 0),
        "snapshot_nodes": parse_int(r"nodes = (\d+)", stdout, 0),
        "contains_ais_stream": "AIS stream" in stdout,
    }


def parse_phase2(stdout: str) -> dict[str, Any]:
    return {
        "html_nodes": parse_int(r"HTML\s+.*?\s+(\d+) nodes", stdout),
        "css_rules": parse_int(r"CSS\s+.*?\s+(\d+) rules", stdout),
        "dom_elements": parse_int(r"DOM\s+.*?\s+(\d+) elements", stdout),
        "layout_primitives": parse_int(r"Layout\s+.*?\s+(\d+) primitives", stdout),
        "display_commands": parse_int(r"Display List\s+.*?\s+(\d+) commands", stdout),
        "output_pixels": parse_int(r"Output\s+.*?\s+(\d+)x600 pixels", stdout),
        "completed": "Phase 2 Demo Complete" in stdout,
    }


def parse_pipeline(stdout: str) -> dict[str, Any]:
    return {
        "web_calls": parse_int(r"script emitted (\d+) WebCalls", stdout),
        "ais_total": parse_int(r"AIS stream:\s+(\d+) instructions", stdout),
        "render_instructions": parse_int(r"\((\d+) render,", stdout),
        "layout_instructions": parse_int(r"render, (\d+) layout,", stdout),
        "state_instructions": parse_int(r"layout, (\d+) state\)", stdout),
        "state_nodes": parse_int(r"nodes=(\d+)", stdout),
        "gpu_skipped": "no GPU adapter available" in stdout,
        "wrote_frame": "wrote out/frame.ppm" in stdout,
    }


def parse_phase4(stdout: str) -> dict[str, Any]:
    return {
        "image_pixels": parse_int(r"Pixel count: (\d+)", stdout),
        "canvas_primitives": len(
            re.findall(r"Generated AIS primitive|fillRect|strokeRect|fillText|drawImage", stdout)
        ),
        "validation_passes": len(re.findall(r"validation: PASS|caught:", stdout)),
        "completed": "Phase 4 Demo Complete" in stdout,
    }


def parse_ppm(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"exists": False}
    raw = path.read_bytes()
    header = raw[:128].split()
    width = height = max_value = None
    if len(header) >= 4 and header[0] == b"P6":
        width = int(header[1])
        height = int(header[2])
        max_value = int(header[3])
    return {
        "exists": True,
        "bytes": len(raw),
        "sha256": hashlib.sha256(raw).hexdigest(),
        "ppm_width": width,
        "ppm_height": height,
        "ppm_max_value": max_value,
    }


def inspect_artifacts(paths: tuple[Path, ...]) -> dict[str, Any]:
    return {str(path.relative_to(ROOT)): parse_ppm(path) for path in paths}


def remove_artifacts(paths: tuple[Path, ...]) -> None:
    for path in paths:
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.exists():
            path.unlink()


def summarize_timings(results: list[IterationResult]) -> dict[str, float]:
    values = [item.elapsed_ms for item in results]
    if not values:
        return {}
    return {
        "min_ms": min(values),
        "max_ms": max(values),
        "mean_ms": statistics.fmean(values),
        "median_ms": statistics.median(values),
        "stdev_ms": statistics.stdev(values) if len(values) > 1 else 0.0,
    }


def merge_numeric_metrics(results: list[IterationResult]) -> dict[str, Any]:
    merged: dict[str, Any] = {}
    keys = sorted({key for item in results for key in item.metrics})
    for key in keys:
        values = [item.metrics.get(key) for item in results]
        if all(isinstance(value, bool) for value in values):
            merged[key] = all(values)
        elif all(isinstance(value, int) for value in values):
            merged[key] = {
                "min": min(values),
                "max": max(values),
                "last": values[-1],
            }
        else:
            merged[key] = values[-1]
    return merged


def run_scenario(
    scenario: Scenario,
    exe: Path,
    iterations: int,
    warmups: int,
    timeout: float,
) -> dict[str, Any]:
    if not exe.exists():
        raise FileNotFoundError(f"missing benchmark binary: {exe}")

    env = os.environ.copy()
    env.setdefault("RUST_BACKTRACE", "1")

    for _ in range(warmups):
        run_command([str(exe)], ROOT, timeout, env=env)

    results: list[IterationResult] = []
    for index in range(iterations):
        remove_artifacts(scenario.artifacts)
        code, elapsed_ms, stdout, stderr = run_command([str(exe)], ROOT, timeout, env=env)
        combined = stdout + "\n" + stderr
        metrics = scenario.parser(combined)
        artifacts = inspect_artifacts(scenario.artifacts)

        if code != 0:
            raise RuntimeError(
                f"{scenario.name} failed with exit code {code}\n"
                f"stdout:\n{stdout}\n"
                f"stderr:\n{stderr}"
            )

        results.append(
            IterationResult(
                iteration=index + 1,
                elapsed_ms=elapsed_ms,
                exit_code=code,
                stdout_bytes=len(stdout.encode()),
                stderr_bytes=len(stderr.encode()),
                metrics=metrics,
                artifacts=artifacts,
                stdout_excerpt=stdout[-1200:],
                stderr_excerpt=stderr[-1200:],
            )
        )

    return {
        "name": scenario.name,
        "binary": scenario.binary,
        "description": scenario.description,
        "summary": summarize_timings(results),
        "metrics": merge_numeric_metrics(results),
        "iterations": [dataclasses.asdict(item) for item in results],
    }


def criterion_smoke(timeout: float) -> dict[str, Any]:
    cmd = [
        "cargo",
        "bench",
        "-p",
        "slate-kernel",
        "--bench",
        "dispatch",
        "--",
        "--sample-size",
        "10",
    ]
    code, elapsed_ms, stdout, stderr = run_command(cmd, ROOT, timeout)
    return {
        "command": cmd,
        "exit_code": code,
        "elapsed_ms": elapsed_ms,
        "criterion_detected": "Benchmarking" in stdout or "time:" in stdout,
        "stdout_excerpt": stdout[-2000:],
        "stderr_excerpt": stderr[-2000:],
    }


def write_reports(report: dict[str, Any], report_dir: Path) -> tuple[Path, Path]:
    report_dir.mkdir(parents=True, exist_ok=True)
    json_path = report_dir / "latest.json"
    md_path = report_dir / "latest.md"

    json_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")

    lines = [
        "# Slate Browser Engine Benchmark Report",
        "",
        f"- Profile: `{report['profile']}`",
        f"- Iterations: `{report['iterations']}`",
        f"- Warmups: `{report['warmups']}`",
        f"- Build time: `{report['build_ms']:.2f} ms`",
        "",
        "## Scenario Summary",
        "",
        "| Scenario | Mean | Median | Min | Max |",
        "| --- | ---: | ---: | ---: | ---: |",
    ]
    for scenario in report["scenarios"]:
        timing = scenario["summary"]
        lines.append(
            "| {name} | {mean:.2f} ms | {median:.2f} ms | {minv:.2f} ms | {maxv:.2f} ms |".format(
                name=scenario["name"],
                mean=timing["mean_ms"],
                median=timing["median_ms"],
                minv=timing["min_ms"],
                maxv=timing["max_ms"],
            )
        )

    lines.extend(["", "## Extracted Engine Metrics", ""])
    for scenario in report["scenarios"]:
        lines.append(f"### {scenario['name']}")
        lines.append("")
        lines.append("```json")
        lines.append(json.dumps(scenario["metrics"], indent=2, sort_keys=True))
        lines.append("```")
        lines.append("")

    md_path.write_text("\n".join(lines) + "\n")
    return json_path, md_path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run detailed Slate browser-engine benchmark scenarios.",
    )
    parser.add_argument("--profile", choices=["debug", "release"], default="release")
    parser.add_argument("--iterations", type=int, default=5)
    parser.add_argument("--warmups", type=int, default=1)
    parser.add_argument("--timeout", type=float, default=120.0)
    parser.add_argument("--build-timeout", type=float, default=600.0)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--criterion-smoke", action="store_true")
    parser.add_argument("--report-dir", type=Path, default=DEFAULT_REPORT_DIR)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.iterations < 1:
        raise SystemExit("--iterations must be at least 1")
    if args.warmups < 0:
        raise SystemExit("--warmups cannot be negative")

    metadata = cargo_metadata(args.timeout)
    target_dir = Path(metadata["target_directory"])
    profile_dir = "release" if args.profile == "release" else "debug"

    scenarios = [
        Scenario(
            name="ais_dispatch_demo",
            binary="slate-demo",
            description="HTML snippet -> WebCall -> AIS -> state snapshot",
            artifacts=(),
            parser=parse_demo,
        ),
        Scenario(
            name="html_css_layout_raster",
            binary="slate-phase2",
            description="HTML/CSS/DOM/flex layout/display-list/CPU raster path",
            artifacts=(ROOT / "output" / "phase2-demo.ppm",),
            parser=parse_phase2,
        ),
        Scenario(
            name="script_kernel_gpu_pipeline",
            binary="slate-pipeline",
            description="Boa script bridge -> kernel -> headless wgpu render path",
            artifacts=(ROOT / "out" / "frame.ppm",),
            parser=parse_pipeline,
            allow_gpu_skip=True,
        ),
        Scenario(
            name="media_forms_svg_surface",
            binary="slate-phase4-demo",
            description="Image, canvas, form validation, and SVG compatibility path",
            artifacts=(),
            parser=parse_phase4,
        ),
    ]

    build_ms = 0.0
    if not args.skip_build:
        build_ms = build_engine(args.profile, args.build_timeout)

    scenario_reports = []
    for scenario in scenarios:
        exe = executable_path(target_dir, profile_dir, scenario.binary)
        scenario_reports.append(
            run_scenario(
                scenario=scenario,
                exe=exe,
                iterations=args.iterations,
                warmups=args.warmups,
                timeout=args.timeout,
            )
        )

    report: dict[str, Any] = {
        "schema_version": 1,
        "generated_at_unix": int(time.time()),
        "root": str(ROOT),
        "target_dir": str(target_dir),
        "profile": args.profile,
        "iterations": args.iterations,
        "warmups": args.warmups,
        "build_ms": build_ms,
        "platform": {
            "system": platform.system(),
            "release": platform.release(),
            "machine": platform.machine(),
            "python": platform.python_version(),
        },
        "scenarios": scenario_reports,
    }

    if args.criterion_smoke:
        report["criterion_smoke"] = criterion_smoke(args.build_timeout)

    json_path, md_path = write_reports(report, args.report_dir)
    print(f"wrote {json_path}")
    print(f"wrote {md_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
