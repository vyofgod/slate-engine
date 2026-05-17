import { SiteNav } from "@/components/site-nav"
import { SiteFooter } from "@/components/site-footer"

export const metadata = {
  title: "Architecture - Slate Engine",
  description: "Architecture boundaries for the Slate Engine active workspace.",
}

const layers = [
  {
    title: "Input surfaces",
    text: "HTML, CSS, script, network, DOM, and compatibility code produce web-facing calls or model data.",
  },
  {
    title: "Dispatcher",
    text: "The dispatcher normalizes WebCall input and decomposes it into AtomicInstruction streams.",
  },
  {
    title: "Kernel and state",
    text: "The kernel submits calls, applies instructions, exposes snapshots, and manages the page-scoped arena.",
  },
  {
    title: "Render and raster",
    text: "Headless wgpu and CPU raster paths produce artifacts that benchmark tooling can inspect.",
  },
]

const boundaries = [
  { owns: "AIS", does: "instruction vocabulary", avoids: "parsing policy or browser compatibility claims" },
  { owns: "Dispatcher", does: "normalization and decomposition", avoids: "global state ownership" },
  { owns: "Kernel", does: "orchestration and replay", avoids: "browser UI behavior" },
  { owns: "Renderer", does: "offscreen rendering and readback", avoids: "DOM or CSS semantics" },
]

export default function ArchitecturePage() {
  return (
    <>
      <SiteNav />
      <main className="min-h-screen bg-background text-foreground">
        <section className="relative min-h-screen flex items-center border-b border-border pt-16">
          <div className="absolute inset-0 grid-lines opacity-20" aria-hidden="true" />
          <div className="relative mx-auto max-w-[1400px] px-6 py-24 md:px-10 w-full">
            <div className="max-w-5xl">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-6">
                Architecture
              </div>
              <h1 className="font-sans text-[3.5rem] leading-[0.95] text-foreground md:text-[6rem] text-balance">
                A narrow internal contract for engine work.
              </h1>
              <p className="mt-8 max-w-3xl font-mono text-sm leading-relaxed text-muted-foreground">
                Slate separates web-facing input, instruction decomposition, state application, and render output.
                The point is inspectability: each stage can be measured and reviewed independently.
              </p>
            </div>
          </div>
        </section>

        <section className="border-b border-border py-24">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Pipeline
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Input to AIS to state to artifact.</h2>
            </div>
            <div className="grid grid-cols-1 gap-px bg-border md:grid-cols-4">
              {layers.map((layer) => (
                <div key={layer.title} className="bg-background p-6">
                  <h3 className="font-sans text-xl text-foreground mb-3">{layer.title}</h3>
                  <p className="font-mono text-xs leading-relaxed text-muted-foreground">{layer.text}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="border-b border-border py-24 bg-muted/20">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Subsystem contracts
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Boundaries are part of the design.</h2>
            </div>
            <div className="border border-border">
              {boundaries.map((row) => (
                <div key={row.owns} className="grid grid-cols-1 gap-4 border-b border-border p-5 last:border-b-0 md:grid-cols-3">
                  <div className="font-mono text-sm text-foreground">{row.owns}</div>
                  <div className="font-mono text-sm text-muted-foreground">{row.does}</div>
                  <div className="font-mono text-sm text-muted-foreground">{row.avoids}</div>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="py-24">
          <div className="mx-auto max-w-[1100px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Workspace boundary
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Active crates are not the same as staged directories.</h2>
            </div>
            <p className="font-mono text-sm leading-relaxed text-muted-foreground">
              The active Cargo workspace is the source of truth for what normal workspace commands verify.
              Directories such as `slate-storage`, `slate-wasm`, `slate-webgl`, `slate-websocket`, and
              `slate-workers` exist in the repository but are currently staged outside the root workspace.
            </p>
          </div>
        </section>
      </main>
      <SiteFooter />
    </>
  )
}
