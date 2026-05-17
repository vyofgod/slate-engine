"use client"

const metrics = [
  { label: "Scenarios", value: "4", detail: "demo pipelines measured" },
  { label: "Reports", value: "JSON + MD", detail: "structured benchmark output" },
  { label: "Artifacts", value: "PPM", detail: "dimensions and hashes validated" },
]

export function BenchmarkSection() {
  return (
    <section id="benchmarks" className="relative border-b border-border overflow-hidden">
      <div className="mx-auto max-w-[1400px] px-6 py-20 md:px-10 w-full">
        <div className="mb-12 flex flex-col md:flex-row md:items-end md:justify-between gap-6">
          <div className="max-w-4xl">
            <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
              Benchmarks
            </div>
            <h2 className="font-sans text-3xl leading-[1.1] text-foreground md:text-4xl text-balance">
              Measure the current engine paths, not imagined browser parity.
            </h2>
            <p className="mt-4 font-mono text-sm leading-relaxed text-muted-foreground max-w-2xl">
              The Python harness builds real Slate binaries, runs demo scenarios, extracts stage metrics, and validates
              generated render artifacts. It does not compare Slate against Chromium, WebKit, or Gecko.
            </p>
          </div>

          <a
            href="/benchmarks"
            className="flex items-center gap-3 font-mono text-xs uppercase tracking-[0.15em] text-foreground hover:text-muted-foreground transition-colors group"
          >
            <span>View Benchmark Notes</span>
            <span className="transition-transform group-hover:translate-x-1">-&gt;</span>
          </a>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {metrics.map((item) => (
            <div key={item.label} className="border border-border p-6">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-2">
                {item.label}
              </div>
              <div className="font-sans text-4xl text-foreground mb-1">{item.value}</div>
              <div className="text-sm text-muted-foreground">{item.detail}</div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
