"use client"

import Link from "next/link"

export function SiteNav() {
  return (
    <header className="fixed top-0 left-0 right-0 z-50 border-b border-border bg-background/95 backdrop-blur-md">
      <div className="mx-auto flex max-w-[1400px] items-center justify-between px-4 py-2 md:px-6">
        <Link href="/" className="flex items-center gap-3">
          <span className="font-orbitron text-2xl font-black tracking-[0.25em] text-foreground">SLATE</span>
          <span className="text-muted-foreground text-xl font-light">/</span>
          <img src="/slate-logo.svg" alt="Slate Logo" className="h-6 w-6 object-contain" />
        </Link>

        <nav className="flex items-center gap-6">
          {[
            { label: "ABOUT", href: "/about" },
            { label: "ARCHITECTURE", href: "/architecture" },
            { label: "BENCHMARKS", href: "/benchmarks" },
            { label: "SOURCE", href: "/docs" },
          ].map((item) => (
            <Link
              key={item.label}
              href={item.href}
              className="font-mono text-xs uppercase tracking-[0.15em] text-muted-foreground transition-colors hover:text-foreground"
            >
              {item.label}
            </Link>
          ))}
        </nav>
      </div>
    </header>
  )
}
