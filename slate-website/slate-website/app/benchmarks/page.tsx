import { SiteNav } from "@/components/site-nav"
import { SiteFooter } from "@/components/site-footer"

export const metadata = {
  title: "Benchmarks - Slate Engine",
  description: "Benchmark harness notes for Slate Engine demo pipelines and generated artifacts.",
}

const scenarios = [
  {
    name: "ais_dispatch_demo",
    binary: "slate-demo",
    purpose: "HTML-style sample to WebCall, AIS stream, and state snapshot.",
  },
  {
    name: "html_css_layout_raster",
    binary: "slate-phase2",
    purpose: "HTML, CSS, DOM, layout, display list, and CPU raster artifact.",
  },
  {
    name: "script_kernel_gpu_pipeline",
    binary: "slate-pipeline",
    purpose: "Script runtime, kernel dispatch, and headless GPU render path.",
  },
  {
    name: "media_forms_svg_surface",
    binary: "slate-phase4-demo",
    purpose: "Image, canvas-like, form validation, and SVG compatibility surface.",
  },
]

const reportFields = [
  "profile, iterations, warmups, and build time",
  "per-scenario timing summary",
  "extracted engine metrics from stdout/stderr",
  "artifact existence, dimensions, byte size, and SHA-256 hash",
  "GPU skip status when a compatible adapter is not available",
]

export default function BenchmarksPage() {
  return (
    <>
      <SiteNav />
      <main className="min-h-screen bg-background text-foreground">
        <section className="relative min-h-screen flex items-center border-b border-border pt-16">
          <div className="absolute inset-0 grid-lines opacity-20" aria-hidden="true" />
          <div className="relative mx-auto max-w-[1400px] px-6 py-24 md:px-10 w-full">
            <div className="max-w-5xl">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-6">
                Benchmarks
              </div>
              <h1 className="font-sans text-[3.5rem] leading-[0.95] text-foreground md:text-[6rem] text-balance">
                Measure the current engine paths.
              </h1>
              <p className="mt-8 max-w-3xl font-mono text-sm leading-relaxed text-muted-foreground">
                Slate benchmark documentation focuses on the repository’s own binaries and artifacts. It does not claim
                external browser-engine performance wins without independently reproduced measurements.
              </p>
            </div>
          </div>
        </section>

        <section className="border-b border-border py-24">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Primary command
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Use the Python harness.</h2>
            </div>
            <pre className="overflow-x-auto border border-border bg-secondary/20 p-6 font-mono text-xs text-muted-foreground">
{`python3 scripts/browser_engine_benchmark.py --profile debug --iterations 1 --warmups 0
python3 scripts/browser_engine_benchmark.py --profile release --iterations 10 --warmups 2`}
            </pre>
          </div>
        </section>

        <section className="border-b border-border py-24 bg-muted/20">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Scenarios
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Four demo paths, one structured report.</h2>
            </div>
            <div className="grid grid-cols-1 gap-px bg-border md:grid-cols-2">
              {scenarios.map((scenario) => (
                <div key={scenario.name} className="bg-background p-6">
                  <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-3">
                    {scenario.binary}
                  </div>
                  <h3 className="font-sans text-xl text-foreground mb-2">{scenario.name}</h3>
                  <p className="font-mono text-xs leading-relaxed text-muted-foreground">{scenario.purpose}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="py-24">
          <div className="mx-auto max-w-[1100px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Report contents
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">What the report proves.</h2>
            </div>
            <div className="space-y-4">
              {reportFields.map((field) => (
                <div key={field} className="border border-border p-5 font-mono text-sm text-muted-foreground">
                  {field}
                </div>
              ))}
            </div>
            <p className="mt-10 font-mono text-sm leading-relaxed text-muted-foreground">
              The generated files are written to `target/slate-browser-benchmark/latest.json` and
              `target/slate-browser-benchmark/latest.md`. Treat debug runs as functional smoke checks and release runs
              as timing data.
            </p>
          </div>
        </section>
      </main>
      <SiteFooter />
    </>
  )
}
