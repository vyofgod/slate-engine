"use client"

const modules = [
  {
    id: "01",
    name: "Input surfaces",
    desc: "HTML, CSS, script, network, DOM, and compatibility code produce web-facing calls or model data.",
  },
  {
    id: "02",
    name: "Dispatcher",
    desc: "WebCall values are normalized and decomposed into AtomicInstruction streams.",
  },
  {
    id: "03",
    name: "Kernel and state",
    desc: "The kernel applies instructions, exposes snapshots, and owns page-scoped arena lifecycle.",
  },
  {
    id: "04",
    name: "Render paths",
    desc: "Headless wgpu and CPU raster paths produce artifacts that can be validated by benchmarks.",
  },
]

export function ArchitectureSection() {
  return (
    <section id="architecture" className="relative border-b border-border overflow-hidden">
      <div className="mx-auto max-w-[1400px] px-6 py-20 md:px-10 w-full">
        <div className="mb-12 max-w-4xl">
          <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
            Architecture
          </div>
          <h2 className="font-sans text-3xl leading-[1.1] text-foreground md:text-4xl text-balance">
            A visible path from input to instruction stream to artifact.
          </h2>
        </div>

        <div className="mt-12 grid grid-cols-1 gap-px bg-border md:grid-cols-2">
          {modules.map((m) => (
            <div key={m.id} className="bg-background p-6">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-3">
                {m.id}
              </div>
              <h3 className="font-sans text-xl text-foreground mb-2">{m.name}</h3>
              <p className="font-mono text-xs leading-relaxed text-muted-foreground">{m.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
