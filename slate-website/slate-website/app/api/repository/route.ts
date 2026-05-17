import { NextResponse } from "next/server"
import { buildRepositoryManifest, readRepositoryEntry } from "@/lib/repository-explorer"

export const runtime = "nodejs"
export const dynamic = "force-dynamic"

export async function GET(request: Request) {
  const url = new URL(request.url)
  const requestedPath = url.searchParams.get("path")
  const wantsRaw = url.searchParams.get("raw") === "1"

  try {
    if (requestedPath !== null) {
      const entry = await readRepositoryEntry(requestedPath)
      if (wantsRaw && entry.type === "file") {
        return new Response(entry.content ?? "", {
          headers: {
            "Content-Type": entry.isBinary ? "application/octet-stream" : "text/plain; charset=utf-8",
            "Content-Disposition": `inline; filename="${entry.name.replace(/"/g, "")}"`,
          },
        })
      }
      return NextResponse.json(entry)
    }

    const manifest = await buildRepositoryManifest()
    return NextResponse.json(manifest)
  } catch (error) {
    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : "Repository explorer request failed",
      },
      { status: 400 },
    )
  }
}
