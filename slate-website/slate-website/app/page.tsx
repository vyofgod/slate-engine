import { SiteNav } from "@/components/site-nav"
import { HeroSection } from "@/components/hero-section"
import { ManifestoSection } from "@/components/manifesto-section"
import { ArchitectureSection } from "@/components/architecture-section"
import { BenchmarkSection } from "@/components/benchmark-section"
import { SiteFooter } from "@/components/site-footer"

export default function Page() {
  return (
    <main className="min-h-screen bg-background text-foreground overflow-x-hidden">
      <SiteNav />
      <HeroSection />
      <ManifestoSection />
      <ArchitectureSection />
      <BenchmarkSection />
      <SiteFooter />
    </main>
  )
}
