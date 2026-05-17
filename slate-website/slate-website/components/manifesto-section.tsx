"use client"

export function ManifestoSection() {
  return (
    <section className="relative border-b border-border overflow-hidden">
      <div className="mx-auto max-w-[1400px] px-6 py-20 md:px-10 w-full">
        <div className="max-w-4xl">
          <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-8">
            Project stance
          </div>
          <p className="font-sans text-xl leading-[1.3] tracking-tight text-foreground md:text-2xl text-balance">
            Slate is documented as an implementation workspace, not as a product claim. The site should tell a
            contributor what the code can currently build, run, measure, and verify.
          </p>
          <p className="mt-4 font-sans text-xl leading-[1.3] tracking-tight text-muted-foreground md:text-2xl text-balance">
            Active workspace members, experimental directories, runnable demos, and benchmark reports are kept
            separate so that source reality stays visible.
          </p>
        </div>
      </div>
    </section>
  )
}
