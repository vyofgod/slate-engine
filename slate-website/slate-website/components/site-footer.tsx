"use client"

const cols = [
  {
    title: "Engine",
    links: [
      { text: "Architecture", href: "/architecture" },
      { text: "Benchmarks", href: "/benchmarks" },
    ],
  },
  {
    title: "Resources",
    links: [
      { text: "Source Explorer", href: "/docs" },
      { text: "GitHub", href: "https://github.com/vyofgod/slate-engine" },
    ],
  },
  {
    title: "Project",
    links: [
      { text: "About", href: "/about" },
      { text: "Repository", href: "https://github.com/vyofgod/slate-engine" },
    ],
  },
]

export function SiteFooter() {
  return (
    <footer className="border-t border-border overflow-hidden">
      <div className="mx-auto max-w-[1400px] px-6 py-16 md:px-10 w-full">
        <div className="grid grid-cols-1 gap-12 lg:grid-cols-12">
          <div className="lg:col-span-4">
            <a href="/" className="flex items-center gap-2">
              <img src="/slate-logo.svg" alt="Slate Logo" className="h-6 w-6 object-contain" />
              <span className="font-mono text-sm font-semibold tracking-[0.2em] text-foreground">SLATE</span>
            </a>
            <p className="mt-4 max-w-xs font-mono text-xs leading-relaxed text-muted-foreground">
              Rust browser-engine architecture workspace. Active crates, staged directories, demos, and benchmarks
              are connected through an inspectable source browser.
            </p>
          </div>

          <div className="grid grid-cols-2 gap-8 sm:grid-cols-3 lg:col-span-8">
            {cols.map((c) => (
              <div key={c.title}>
                <div className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground mb-3">
                  {c.title}
                </div>
                <ul className="space-y-2">
                  {c.links.map((l) => (
                    <li key={l.text}>
                      <a href={l.href} className="font-sans text-sm text-foreground transition-colors hover:text-muted-foreground">
                        {l.text}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>

        <div className="mt-12 border-t border-border pt-6">
          <span className="font-mono text-[11px] uppercase tracking-[0.2em] text-muted-foreground">
            Slate Engine - Apache-2.0 OR MIT
          </span>
        </div>
      </div>
    </footer>
  )
}
