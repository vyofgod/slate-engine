import { SiteNav } from "@/components/site-nav"
import { RepositoryExplorer } from "@/components/repository-explorer"
import { buildRepositoryManifest } from "@/lib/repository-explorer"

export const dynamic = "force-dynamic"

export default async function DocsPage() {
  const manifest = await buildRepositoryManifest()

  return (
    <main className="min-h-screen bg-background text-foreground">
      <SiteNav />
      <RepositoryExplorer manifest={manifest} />
    </main>
  )
}
