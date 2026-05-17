"use client"

const stats = [
  { label: "ACTIVE CRATES", value: "19" },
  { label: "LICENSE", value: "Apache / MIT" },
  { label: "CORE MODEL", value: "AIS" },
  { label: "STATUS", value: "Prototype" },
]

const features = [
  {
    title: "Explicit instruction flow",
    description: "High-level input is reduced into AtomicInstruction streams before state mutation or rendering.",
  },
  {
    title: "Workspace-first documentation",
    description: "The site distinguishes active Cargo workspace members from experimental crate directories.",
  },
  {
    title: "Measurable demos",
    description: "Benchmark tooling runs real Slate binaries, extracts metrics, and validates generated artifacts.",
  },
]

const highlights = [
  { metric: "AIS", label: "Internal IR" },
  { metric: "wgpu", label: "Headless render path" },
  { metric: "PPM", label: "Validated artifacts" },
  { metric: "Rust", label: "Workspace implementation" },
]

export function HeroSection() {
  const handleClick = (e: React.MouseEvent<HTMLAnchorElement>, href: string) => {
    e.preventDefault()
    const element = document.querySelector(href)
    if (element) {
      element.scrollIntoView({ behavior: "smooth", block: "start" })
    }
  }

  return (
    <section className="relative min-h-screen flex items-center overflow-hidden border-b border-border pt-16">
      <div className="absolute inset-0 grid-lines opacity-20" aria-hidden="true" />

      <div className="relative mx-auto max-w-[1400px] px-6 py-20 md:px-10 w-full overflow-hidden">
        <div className="mb-16 flex items-center gap-3 font-mono text-xs uppercase tracking-[0.2em] text-muted-foreground">
          <span className="flex items-center gap-2">
            <span className="h-1 w-1 bg-foreground" />
            SLATE ENGINE
          </span>
          <span className="text-muted-foreground/40">/</span>
          <span className="text-muted-foreground/60">Browser-engine architecture workspace</span>
        </div>

        <div className="grid grid-cols-1 gap-12 lg:grid-cols-12">
          <div className="lg:col-span-7">
            <h1 className="font-sans text-[3rem] leading-[0.95] text-foreground sm:text-[4.5rem] md:text-[5rem] lg:text-[6rem] text-balance">
              EXPLICIT CORE.
              <br />
              HONEST SCOPE.
            </h1>

            <div className="mt-8 grid grid-cols-2 gap-4 lg:hidden">
              {highlights.map((h) => (
                <div key={h.label} className="border border-border/50 bg-background/50 backdrop-blur-sm px-4 py-3">
                  <div className="font-sans text-2xl font-bold text-foreground">{h.metric}</div>
                  <div className="font-mono text-[10px] uppercase tracking-wider text-muted-foreground/70 mt-1">
                    {h.label}
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="flex flex-col justify-end gap-6 lg:col-span-5">
            <div className="space-y-4">
              <p className="max-w-md font-mono text-sm leading-relaxed text-muted-foreground">
                Slate is a Rust workspace for a browser-engine style pipeline built around a narrow Atomic
                Instruction Set. It is an engine prototype and research codebase, not a finished browser.
              </p>

              <p className="max-w-md font-mono text-sm leading-relaxed text-muted-foreground">
                The current site exposes what exists in source today: active workspace crates, staged directories,
                demo binaries, benchmark harnesses, and the boundaries between them.
              </p>
            </div>

            <div className="flex flex-wrap gap-3">
              <a
                href="/architecture"
                className="rounded-2xl border border-border px-4 py-2.5 font-mono text-xs uppercase tracking-[0.15em] text-foreground transition-colors hover:border-foreground cursor-pointer"
              >
                Architecture
              </a>
              <a
                href="#benchmarks"
                onClick={(e) => handleClick(e, "#benchmarks")}
                className="rounded-2xl border border-border px-4 py-2.5 font-mono text-xs uppercase tracking-[0.15em] text-foreground transition-colors hover:border-foreground cursor-pointer"
              >
                Benchmarks
              </a>
            </div>
          </div>
        </div>

        <div className="mt-16 hidden lg:grid grid-cols-4 gap-4">
          {highlights.map((h) => (
            <div key={h.label} className="border border-border/50 bg-background/50 backdrop-blur-sm px-5 py-4 hover:border-foreground/30 transition-colors">
              <div className="font-sans text-3xl font-bold text-foreground">{h.metric}</div>
              <div className="font-mono text-xs uppercase tracking-wider text-muted-foreground mt-2">{h.label}</div>
            </div>
          ))}
        </div>

        <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-6">
          {features.map((feature) => (
            <div key={feature.title} className="border border-border/30 bg-background/30 backdrop-blur-sm p-6 hover:border-foreground/20 transition-colors">
              <h3 className="font-mono text-xs uppercase tracking-[0.15em] text-foreground mb-3">
                {feature.title}
              </h3>
              <p className="font-mono text-sm leading-relaxed text-muted-foreground">{feature.description}</p>
            </div>
          ))}
        </div>

        <div className="mt-20 grid grid-cols-2 gap-px bg-border md:grid-cols-4">
          {stats.map((s) => (
            <div key={s.label} className="bg-background px-5 py-6">
              <div className="font-mono text-xs uppercase tracking-[0.2em] text-muted-foreground mb-2">{s.label}</div>
              <div className="font-sans text-2xl text-foreground md:text-3xl">{s.value}</div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
