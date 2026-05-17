import { SiteNav } from "@/components/site-nav"
import { SiteFooter } from "@/components/site-footer"

export const metadata = {
  title: "About - Slate Engine",
  description: "Project status, scope, and repository boundaries for the Slate Engine workspace.",
}

const principles = [
  "Expose current behavior before future ambition.",
  "Keep active workspace crates separate from staged directories.",
  "Treat benchmark numbers as local measurements, not universal claims.",
  "Describe compatibility layers by exact behavior, not by browser-standard completion.",
]

const scope = [
  { label: "Active workspace", value: "19 crates", detail: "Covered by root Cargo workspace commands." },
  { label: "Core model", value: "AIS", detail: "AtomicInstruction streams connect dispatch, state, and render." },
  { label: "Demos", value: "4 binaries", detail: "slate-demo, slate-phase2, slate-pipeline, slate-phase4-demo." },
  { label: "Benchmarking", value: "Python harness", detail: "Runs real binaries and writes JSON/Markdown reports." },
]

const faqs = [
  {
    q: "Is Slate a finished browser?",
    a: "No. Slate is currently a Rust workspace for browser-engine architecture experiments and implementation work. It has demos, crates, and benchmark tooling, but it should not be described as a finished browser or production WebView.",
  },
  {
    q: "What is AIS?",
    a: "AIS means Atomic Instruction Set. In this repository, it is the internal instruction vocabulary used between dispatcher, state, and rendering paths.",
  },
  {
    q: "Does the project claim Chromium or WebKit performance parity?",
    a: "No. The site intentionally avoids unverified comparisons. Current benchmarks measure Slate demo paths and generated artifacts.",
  },
  {
    q: "Can experimental crate directories be used?",
    a: "They exist in source, but they are not active root workspace members yet. Promote them only after workspace membership, dependency review, checks, tests, and docs are updated.",
  },
]

export default function AboutPage() {
  return (
    <>
      <SiteNav />
      <main className="min-h-screen">
        <section className="relative min-h-screen flex items-center overflow-hidden border-b border-border pt-16">
          <div className="absolute inset-0 grid-lines opacity-20" aria-hidden="true" />
          <div className="relative mx-auto max-w-[1400px] px-6 py-24 md:px-10 w-full">
            <div className="max-w-5xl space-y-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground">
                About the workspace
              </div>
              <h1 className="font-sans text-[3.5rem] leading-[0.95] text-foreground md:text-[6rem] text-balance">
                Slate is an engine workspace with a deliberately explicit core.
              </h1>
              <p className="max-w-3xl font-mono text-sm leading-relaxed text-muted-foreground">
                The project explores a browser-engine style pipeline in Rust. It focuses on reducing high-level input
                into an inspectable instruction stream, applying that stream through a kernel/state layer, and measuring
                real demo paths without overstating browser compatibility.
              </p>
            </div>

            <div className="mt-16 grid grid-cols-1 gap-px bg-border md:grid-cols-4">
              {scope.map((item) => (
                <div key={item.label} className="bg-background p-6">
                  <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-3">
                    {item.label}
                  </div>
                  <div className="font-sans text-2xl text-foreground mb-2">{item.value}</div>
                  <p className="font-mono text-xs leading-relaxed text-muted-foreground">{item.detail}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="border-b border-border py-24">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="grid grid-cols-1 gap-12 lg:grid-cols-12">
              <div className="lg:col-span-4">
                <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                  Why it exists
                </div>
                <h2 className="font-sans text-3xl text-foreground md:text-4xl">
                  A smaller internal contract for complex engine work.
                </h2>
              </div>
              <div className="space-y-6 lg:col-span-8">
                <p className="font-mono text-sm leading-relaxed text-muted-foreground">
                  Browser-engine work is usually difficult because parsing, scripting, layout, state, rendering,
                  networking, and platform behavior can become tightly coupled. Slate keeps those boundaries visible by
                  routing work through explicit calls, instruction streams, kernel application, and artifact-producing
                  render paths.
                </p>
                <p className="font-mono text-sm leading-relaxed text-muted-foreground">
                  That does not make Slate complete by itself. It makes the current implementation easier to inspect,
                  test, document, and benchmark as it grows.
                </p>
              </div>
            </div>
          </div>
        </section>

        <section className="border-b border-border py-24 bg-muted/20">
          <div className="mx-auto max-w-[1400px] px-6 md:px-10">
            <div className="mb-10">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                Source principles
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">No imaginary product claims.</h2>
            </div>
            <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
              {principles.map((item) => (
                <div key={item} className="border border-border bg-background p-6 font-mono text-sm text-muted-foreground">
                  {item}
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="py-24">
          <div className="mx-auto max-w-[1000px] px-6 md:px-10">
            <div className="mb-12">
              <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-4">
                FAQ
              </div>
              <h2 className="font-sans text-3xl text-foreground md:text-4xl">Useful boundaries.</h2>
            </div>
            <div className="space-y-8">
              {faqs.map((item) => (
                <div key={item.q} className="border-b border-border pb-8">
                  <h3 className="font-sans text-xl text-foreground mb-3">{item.q}</h3>
                  <p className="font-mono text-sm leading-relaxed text-muted-foreground">{item.a}</p>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>
      <SiteFooter />
    </>
  )
}
