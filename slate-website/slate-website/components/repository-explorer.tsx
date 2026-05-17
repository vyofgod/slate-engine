"use client"

import type { RepositoryEntry, RepositoryManifest, RepositoryNode } from "@/lib/repository-explorer"
import {
  BarChart3,
  Binary,
  BookOpen,
  ChevronDown,
  ChevronRight,
  Copy,
  Download,
  Eye,
  ExternalLink,
  FileCode2,
  FileText,
  Folder,
  FolderOpen,
  GitBranch,
  ListTree,
  Search,
  TerminalSquare,
} from "lucide-react"
import { useEffect, useMemo, useState } from "react"
import ReactMarkdown from "react-markdown"
import remarkGfm from "remark-gfm"

type ExplorerProps = {
  manifest: RepositoryManifest
}

type SelectedEntry = RepositoryEntry | null
type ExplorerTab = "code" | "readme" | "insights"

const quickFilters = ["Rust", "Markdown", "TOML", "TypeScript", "crates/", "benchmark"]

function formatNumber(value: number) {
  return new Intl.NumberFormat("en-US").format(value)
}

function formatBytes(value: number) {
  if (value < 1024) return `${value} B`
  if (value < 1024 * 1024) return `${(value / 1024).toFixed(1)} KB`
  return `${(value / 1024 / 1024).toFixed(1)} MB`
}

function flattenTree(node: RepositoryNode): RepositoryNode[] {
  const children = node.children ?? []
  return [node, ...children.flatMap(flattenTree)]
}

function findNode(node: RepositoryNode, selectedPath: string): RepositoryNode | null {
  if (node.path === selectedPath) return node
  for (const child of node.children ?? []) {
    const match = findNode(child, selectedPath)
    if (match) return match
  }
  return null
}

function getParentPaths(filePath: string) {
  const segments = filePath.split("/").filter(Boolean)
  const parents: string[] = [""]

  for (let index = 1; index < segments.length; index += 1) {
    parents.push(segments.slice(0, index).join("/"))
  }

  return parents
}

function getLanguageTone(language?: string) {
  switch (language) {
    case "Rust":
      return "border-orange-400/40 bg-orange-400/10 text-orange-100"
    case "Markdown":
      return "border-sky-300/40 bg-sky-300/10 text-sky-100"
    case "TypeScript":
    case "TypeScript React":
      return "border-blue-300/40 bg-blue-300/10 text-blue-100"
    case "TOML":
      return "border-emerald-300/40 bg-emerald-300/10 text-emerald-100"
    case "Shell":
      return "border-lime-300/40 bg-lime-300/10 text-lime-100"
    default:
      return "border-border bg-secondary/30 text-muted-foreground"
  }
}

function getFileIcon(node: RepositoryNode) {
  if (node.type === "directory") return <Folder className="h-4 w-4 text-muted-foreground" />
  if (node.isBinary) return <Binary className="h-4 w-4 text-muted-foreground" />
  if (node.language === "Markdown") return <FileText className="h-4 w-4 text-muted-foreground" />
  return <FileCode2 className="h-4 w-4 text-muted-foreground" />
}

function findReadme(node: RepositoryNode) {
  return (node.children ?? []).find((child) => child.type === "file" && child.name.toLowerCase() === "readme.md") ?? null
}

function TreeNode({
  node,
  depth,
  selectedPath,
  expandedPaths,
  onSelect,
  onToggle,
}: {
  node: RepositoryNode
  depth: number
  selectedPath: string
  expandedPaths: Set<string>
  onSelect: (node: RepositoryNode) => void
  onToggle: (path: string) => void
}) {
  const isDirectory = node.type === "directory"
  const isExpanded = expandedPaths.has(node.path)
  const isSelected = selectedPath === node.path
  const children = node.children ?? []

  return (
    <div>
      <button
        type="button"
        onClick={() => {
          if (isDirectory) onToggle(node.path)
          onSelect(node)
        }}
        className={`group flex w-full items-center gap-2 px-2 py-1.5 text-left font-mono text-[12px] transition-colors ${
          isSelected
            ? "bg-foreground text-background"
            : "text-muted-foreground hover:bg-secondary/60 hover:text-foreground"
        }`}
        style={{ paddingLeft: `${depth * 13 + 8}px` }}
      >
        <span className="flex h-4 w-4 shrink-0 items-center justify-center">
          {isDirectory ? (
            isExpanded ? (
              <ChevronDown className="h-3.5 w-3.5" />
            ) : (
              <ChevronRight className="h-3.5 w-3.5" />
            )
          ) : (
            <span className="h-3.5 w-3.5" />
          )}
        </span>
        {isDirectory ? (
          isExpanded ? (
            <FolderOpen className="h-3.5 w-3.5 shrink-0" />
          ) : (
            <Folder className="h-3.5 w-3.5 shrink-0" />
          )
        ) : (
          <FileCode2 className="h-3.5 w-3.5 shrink-0" />
        )}
        <span className="min-w-0 flex-1 truncate">{node.name}</span>
      </button>

      {isDirectory && isExpanded
        ? children.map((child) => (
            <TreeNode
              key={child.path}
              node={child}
              depth={depth + 1}
              selectedPath={selectedPath}
              expandedPaths={expandedPaths}
              onSelect={onSelect}
              onToggle={onToggle}
            />
          ))
        : null}
    </div>
  )
}

function Breadcrumbs({
  path,
  onSelectPath,
}: {
  path: string
  onSelectPath: (path: string) => void
}) {
  const segments = path.split("/").filter(Boolean)

  return (
    <div className="flex min-w-0 flex-wrap items-center gap-1 font-mono text-sm">
      <button type="button" onClick={() => onSelectPath("")} className="text-foreground hover:underline">
        slate-engine
      </button>
      {segments.map((segment, index) => {
        const segmentPath = segments.slice(0, index + 1).join("/")
        return (
          <span key={segmentPath} className="flex min-w-0 items-center gap-1">
            <span className="text-muted-foreground/50">/</span>
            <button
              type="button"
              onClick={() => onSelectPath(segmentPath)}
              className="max-w-[220px] truncate text-muted-foreground hover:text-foreground hover:underline"
            >
              {segment}
            </button>
          </span>
        )
      })}
    </div>
  )
}

function RepositoryHeader({ manifest }: { manifest: RepositoryManifest }) {
  const topLanguages = manifest.stats.languages.slice(0, 4)
  const totalLanguageLines = topLanguages.reduce((total, language) => total + language.lines, 0) || 1
  const latestCommit = manifest.recentCommits[0]

  return (
    <section className="border border-border bg-background">
      <div className="flex flex-col gap-5 border-b border-border p-5 lg:flex-row lg:items-start lg:justify-between">
        <div className="min-w-0">
          <div className="mb-3 flex flex-wrap items-center gap-2 font-mono text-xs uppercase tracking-[0.16em] text-muted-foreground">
            <GitBranch className="h-4 w-4" />
            Source Explorer
            <span className="border border-border px-2 py-1 text-[10px] text-muted-foreground">main</span>
            <span className="border border-border px-2 py-1 text-[10px] text-muted-foreground">local index</span>
          </div>
          <h1 className="truncate font-sans text-3xl text-foreground md:text-4xl">vyofgod / slate-engine</h1>
          <p className="mt-3 max-w-3xl font-mono text-sm leading-relaxed text-muted-foreground">
            Browse Slate Engine like a repository: source tree, file metadata, line-numbered previews, and direct GitHub
            links in one focused interface.
          </p>
          {latestCommit ? (
            <div className="mt-4 flex max-w-3xl flex-wrap items-center gap-2 border border-border bg-secondary/20 px-3 py-2 font-mono text-xs text-muted-foreground">
              <span className="text-foreground">{latestCommit.hash}</span>
              <span className="truncate">{latestCommit.subject}</span>
              <span className="text-muted-foreground/50">by {latestCommit.author}</span>
              <span className="text-muted-foreground/50">{latestCommit.date}</span>
            </div>
          ) : null}
        </div>
        <a
          href="https://github.com/vyofgod/slate-engine"
          target="_blank"
          rel="noreferrer"
          className="inline-flex shrink-0 items-center justify-center gap-2 border border-border px-4 py-2.5 font-mono text-xs uppercase tracking-[0.14em] text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
        >
          <ExternalLink className="h-4 w-4" />
          GitHub
        </a>
      </div>

      <div className="grid gap-px bg-border sm:grid-cols-2 lg:grid-cols-4">
        {[
          { label: "Files", value: formatNumber(manifest.stats.files) },
          { label: "Directories", value: formatNumber(manifest.stats.directories) },
          { label: "Text Lines", value: formatNumber(manifest.stats.totalLines) },
          { label: "Indexed Size", value: formatBytes(manifest.stats.totalBytes) },
        ].map((item) => (
          <div key={item.label} className="bg-background px-5 py-4">
            <div className="font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">{item.label}</div>
            <div className="mt-1 font-sans text-2xl text-foreground">{item.value}</div>
          </div>
        ))}
      </div>

      <div className="space-y-3 p-5">
        <div className="flex h-2 overflow-hidden border border-border bg-secondary/20">
          {topLanguages.map((language) => (
            <div
              key={language.language}
              className="border-r border-background bg-foreground/80 last:border-r-0"
              style={{ width: `${Math.max(4, (language.lines / totalLanguageLines) * 100)}%` }}
              title={`${language.language}: ${formatNumber(language.lines)} lines`}
            />
          ))}
        </div>
        <div className="flex flex-wrap gap-2">
          {topLanguages.map((language) => (
            <span key={language.language} className={`border px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.14em] ${getLanguageTone(language.language)}`}>
              {language.language} · {formatNumber(language.files)}
            </span>
          ))}
        </div>
      </div>
    </section>
  )
}

function Toolbar({
  query,
  searchCount,
  onQueryChange,
  onSelectPath,
}: {
  query: string
  searchCount: number
  onQueryChange: (query: string) => void
  onSelectPath: (path: string) => void
}) {
  return (
    <div className="border border-border bg-background p-3">
      <div className="flex flex-col gap-3 lg:flex-row lg:items-center">
        <label className="flex min-w-0 flex-1 items-center gap-3 border border-border bg-secondary/20 px-3 py-2.5">
          <Search className="h-4 w-4 text-muted-foreground" />
          <input
            value={query}
            onChange={(event) => onQueryChange(event.target.value)}
            placeholder="Search files, paths, languages..."
            className="w-full bg-transparent font-mono text-sm text-foreground outline-none placeholder:text-muted-foreground"
          />
        </label>
        <div className="flex flex-wrap gap-2">
          {quickFilters.map((filter) => (
            <button
              key={filter}
              type="button"
              onClick={() => onQueryChange(filter)}
              className="border border-border px-3 py-2 font-mono text-[10px] uppercase tracking-[0.14em] text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
            >
              {filter}
            </button>
          ))}
          <button
            type="button"
            onClick={() => onSelectPath("README.md")}
            className="border border-border px-3 py-2 font-mono text-[10px] uppercase tracking-[0.14em] text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
          >
            README
          </button>
        </div>
      </div>
      {query.trim() ? (
        <div className="mt-3 font-mono text-xs text-muted-foreground">{formatNumber(searchCount)} matching entries</div>
      ) : null}
    </div>
  )
}

function RepositoryTabs({
  activeTab,
  onTabChange,
  hasReadme,
}: {
  activeTab: ExplorerTab
  onTabChange: (tab: ExplorerTab) => void
  hasReadme: boolean
}) {
  const tabs: Array<{ id: ExplorerTab; label: string; icon: typeof ListTree; disabled?: boolean }> = [
    { id: "code", label: "Code", icon: ListTree },
    { id: "readme", label: "README", icon: BookOpen, disabled: !hasReadme },
    { id: "insights", label: "Insights", icon: BarChart3 },
  ]

  return (
    <div className="flex flex-wrap border border-border bg-background">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          type="button"
          disabled={tab.disabled}
          onClick={() => onTabChange(tab.id)}
          className={`inline-flex items-center gap-2 border-r border-border px-4 py-3 font-mono text-xs transition-colors last:border-r-0 ${
            activeTab === tab.id
              ? "bg-foreground text-background"
              : "text-muted-foreground hover:bg-secondary/40 hover:text-foreground"
          } ${tab.disabled ? "cursor-not-allowed opacity-40 hover:bg-transparent hover:text-muted-foreground" : ""}`}
        >
          <tab.icon className="h-4 w-4" />
          {tab.label}
        </button>
      ))}
    </div>
  )
}

function ReadmePanel({ entry }: { entry: Extract<RepositoryEntry, { type: "file" }> | null }) {
  if (!entry) {
    return (
      <div className="border border-border bg-background p-8 font-mono text-sm text-muted-foreground">
        No README.md file exists in this directory.
      </div>
    )
  }

  return (
    <article className="border border-border bg-background">
      <div className="flex items-center gap-2 border-b border-border bg-secondary/20 px-4 py-3 font-mono text-sm text-foreground">
        <BookOpen className="h-4 w-4" />
        {entry.path}
      </div>
      <div className="prose prose-invert max-w-none p-6 prose-headings:font-sans prose-p:font-mono prose-p:text-sm prose-p:leading-relaxed prose-li:font-mono prose-li:text-sm prose-code:text-foreground prose-pre:border prose-pre:border-border prose-pre:bg-[#0b0b0b]">
        <ReactMarkdown remarkPlugins={[remarkGfm]}>{entry.content ?? ""}</ReactMarkdown>
      </div>
    </article>
  )
}

function InsightsPanel({ manifest }: { manifest: RepositoryManifest }) {
  const topLanguages = manifest.stats.languages.slice(0, 10)
  const maxLines = Math.max(...topLanguages.map((language) => language.lines), 1)

  return (
    <div className="grid gap-4 lg:grid-cols-[1fr_340px]">
      <div className="border border-border bg-background">
        <div className="border-b border-border bg-secondary/20 px-4 py-3 font-mono text-sm text-foreground">
          Language breakdown
        </div>
        <div className="divide-y divide-border">
          {topLanguages.map((language) => (
            <div key={language.language} className="grid gap-3 px-4 py-4 md:grid-cols-[180px_1fr_160px] md:items-center">
              <div className="font-mono text-sm text-foreground">{language.language}</div>
              <div className="h-2 border border-border bg-secondary/20">
                <div className="h-full bg-foreground" style={{ width: `${Math.max(3, (language.lines / maxLines) * 100)}%` }} />
              </div>
              <div className="font-mono text-xs text-muted-foreground md:text-right">
                {formatNumber(language.files)} files · {formatNumber(language.lines)} lines
              </div>
            </div>
          ))}
        </div>
      </div>

      <aside className="space-y-4">
        <div className="border border-border bg-background">
          <div className="border-b border-border bg-secondary/20 px-4 py-3 font-mono text-sm text-foreground">
            Recent commits
          </div>
          <div className="divide-y divide-border">
            {manifest.recentCommits.length ? (
              manifest.recentCommits.map((commit) => (
                <div key={commit.hash} className="p-4">
                  <div className="font-mono text-xs text-foreground">{commit.subject}</div>
                  <div className="mt-2 flex flex-wrap gap-2 font-mono text-[11px] text-muted-foreground">
                    <span>{commit.hash}</span>
                    <span>{commit.author}</span>
                    <span>{commit.date}</span>
                  </div>
                </div>
              ))
            ) : (
              <div className="p-4 font-mono text-xs text-muted-foreground">No local git history was available.</div>
            )}
          </div>
        </div>

        <div className="border border-border bg-background p-4">
          <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">
            Indexed Scope
          </div>
          <div className="grid grid-cols-2 gap-px bg-border">
            {[
              ["Text files", formatNumber(manifest.stats.textFiles)],
              ["Binary files", formatNumber(manifest.stats.binaryFiles)],
              ["Total bytes", formatBytes(manifest.stats.totalBytes)],
              ["Generated", manifest.generatedAt.slice(0, 10)],
            ].map(([label, value]) => (
              <div key={label} className="bg-background p-3">
                <div className="font-mono text-[10px] uppercase tracking-[0.14em] text-muted-foreground">{label}</div>
                <div className="mt-1 font-mono text-sm text-foreground">{value}</div>
              </div>
            ))}
          </div>
        </div>

        <div className="border border-border bg-background p-4">
          <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">
            Excluded From Browser
          </div>
          <div className="flex flex-wrap gap-2">
            {manifest.excluded.map((item) => (
              <span key={item} className="border border-border px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.14em] text-muted-foreground">
                {item}
              </span>
            ))}
          </div>
        </div>
      </aside>
    </div>
  )
}

function DetailsPanel({
  node,
  entry,
  manifest,
  onSelectPath,
}: {
  node: RepositoryNode
  entry: SelectedEntry
  manifest: RepositoryManifest
  onSelectPath: (path: string) => void
}) {
  const siblings = node.path ? node.path.split("/").length - 1 : 0

  return (
    <aside className="space-y-4">
      <div className="border border-border bg-background p-4">
        <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">About</div>
        <div className="space-y-3 font-mono text-xs text-muted-foreground">
          <div className="flex justify-between gap-4">
            <span>Path depth</span>
            <span className="text-foreground">{formatNumber(siblings)}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span>Entry type</span>
            <span className="text-foreground">{node.type}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span>Size</span>
            <span className="text-foreground">{formatBytes(node.size)}</span>
          </div>
          {node.type === "file" ? (
            <>
              <div className="flex justify-between gap-4">
                <span>Language</span>
                <span className="text-foreground">{node.language ?? "Unknown"}</span>
              </div>
              <div className="flex justify-between gap-4">
                <span>Lines</span>
                <span className="text-foreground">{formatNumber(node.lines ?? 0)}</span>
              </div>
            </>
          ) : (
            <div className="flex justify-between gap-4">
              <span>Children</span>
              <span className="text-foreground">{formatNumber(node.children?.length ?? 0)}</span>
            </div>
          )}
        </div>
      </div>

      <div className="border border-border bg-background p-4">
        <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">Repository</div>
        <div className="space-y-2">
          {manifest.featuredPaths.map((pathName) => (
            <button
              key={pathName}
              type="button"
              onClick={() => onSelectPath(pathName)}
              className="block w-full truncate border border-border/70 px-3 py-2 text-left font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
            >
              {pathName}
            </button>
          ))}
        </div>
      </div>

      {entry?.type === "file" && entry.content ? (
        <div className="border border-border bg-background p-4">
          <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">Preview</div>
          <p className="line-clamp-6 font-mono text-xs leading-relaxed text-muted-foreground">
            {entry.content.slice(0, 520)}
          </p>
        </div>
      ) : null}
    </aside>
  )
}

function DirectoryPanel({
  node,
  onSelect,
}: {
  node: RepositoryNode
  onSelect: (node: RepositoryNode) => void
}) {
  const children = node.children ?? []
  const fileCount = children.filter((child) => child.type === "file").length
  const directoryCount = children.length - fileCount

  return (
    <div className="overflow-hidden border border-border bg-background">
      <div className="flex flex-col gap-2 border-b border-border bg-secondary/20 px-4 py-3 md:flex-row md:items-center md:justify-between">
        <div className="font-mono text-sm text-foreground">
          {children.length ? `${formatNumber(children.length)} entries` : "Empty directory"}
        </div>
        <div className="font-mono text-xs text-muted-foreground">
          {formatNumber(directoryCount)} dirs · {formatNumber(fileCount)} files · {formatBytes(node.size)}
        </div>
      </div>

      <div className="divide-y divide-border">
        {children.map((child) => (
          <button
            key={child.path}
            type="button"
            onClick={() => onSelect(child)}
            className="grid w-full grid-cols-[1fr_auto] gap-4 px-4 py-3 text-left transition-colors hover:bg-secondary/40 md:grid-cols-[1fr_130px_100px]"
          >
            <span className="flex min-w-0 items-center gap-3">
              {getFileIcon(child)}
              <span className="min-w-0">
                <span className="block truncate font-mono text-sm text-foreground">{child.name}</span>
                <span className="mt-0.5 block truncate font-mono text-[11px] text-muted-foreground">
                  {child.path || "repository root"}
                </span>
              </span>
            </span>
            <span className="hidden self-center font-mono text-xs text-muted-foreground md:block">
              {child.type === "directory" ? "Directory" : child.language ?? "File"}
            </span>
            <span className="self-center text-right font-mono text-xs text-muted-foreground">
              {child.type === "directory" ? formatNumber(child.children?.length ?? 0) : formatBytes(child.size)}
            </span>
          </button>
        ))}
      </div>
    </div>
  )
}

function FilePanel({ entry }: { entry: Extract<RepositoryEntry, { type: "file" }> | null }) {
  const [copied, setCopied] = useState(false)
  const [copiedContent, setCopiedContent] = useState(false)

  if (!entry) {
    return (
      <div className="border border-border bg-background p-8 font-mono text-sm text-muted-foreground">
        Select a file to inspect its source.
      </div>
    )
  }

  const lines = entry.content?.split("\n") ?? []
  const githubUrl = `https://github.com/vyofgod/slate-engine/blob/main/${entry.path}`

  const copyPath = async () => {
    await navigator.clipboard.writeText(entry.path)
    setCopied(true)
    window.setTimeout(() => setCopied(false), 1200)
  }

  const copyContent = async () => {
    if (!entry.content) return
    await navigator.clipboard.writeText(entry.content)
    setCopiedContent(true)
    window.setTimeout(() => setCopiedContent(false), 1200)
  }

  const rawHref = `/api/repository?raw=1&path=${encodeURIComponent(entry.path)}`

  return (
    <div className="overflow-hidden border border-border bg-background">
      <div className="flex flex-col gap-4 border-b border-border bg-secondary/20 p-4 lg:flex-row lg:items-center lg:justify-between">
        <div className="min-w-0">
          <div className="mb-2 flex flex-wrap items-center gap-2">
            <span className={`border px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.16em] ${getLanguageTone(entry.language)}`}>
              {entry.language}
            </span>
            <span className="border border-border bg-background px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.16em] text-muted-foreground">
              {formatBytes(entry.size)}
            </span>
            <span className="border border-border bg-background px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.16em] text-muted-foreground">
              {formatNumber(entry.lines)} lines
            </span>
            {entry.truncated ? (
              <span className="border border-amber-300/40 bg-amber-300/10 px-2.5 py-1 font-mono text-[10px] uppercase tracking-[0.16em] text-amber-100">
                Truncated
              </span>
            ) : null}
          </div>
          <h2 className="truncate font-mono text-base text-foreground">{entry.path}</h2>
        </div>

        <div className="flex flex-wrap gap-2">
          <button
            type="button"
            onClick={copyPath}
            className="inline-flex items-center gap-2 border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
          >
            <Copy className="h-3.5 w-3.5" />
            {copied ? "Copied" : "Copy path"}
          </button>
          {!entry.isBinary ? (
            <button
              type="button"
              onClick={copyContent}
              className="inline-flex items-center gap-2 border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
            >
              <Copy className="h-3.5 w-3.5" />
              {copiedContent ? "Copied" : "Copy file"}
            </button>
          ) : null}
          <a
            href={rawHref}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-2 border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
          >
            <Eye className="h-3.5 w-3.5" />
            Raw
          </a>
          <a
            href={rawHref}
            download={entry.name}
            className="inline-flex items-center gap-2 border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
          >
            <Download className="h-3.5 w-3.5" />
            Download
          </a>
          <a
            href={githubUrl}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-2 border border-border bg-background px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
          >
            <ExternalLink className="h-3.5 w-3.5" />
            Open on GitHub
          </a>
        </div>
      </div>

      {entry.isBinary ? (
        <div className="p-8">
          <div className="border border-border bg-secondary/20 p-8 text-center">
            <Binary className="mx-auto h-8 w-8 text-muted-foreground" />
            <p className="mt-4 font-mono text-sm text-muted-foreground">
              Binary files are indexed with metadata, but raw bytes are not rendered in the browser.
            </p>
          </div>
        </div>
      ) : (
        <div className="max-h-[760px] overflow-auto bg-[#0b0b0b]">
          <pre className="min-w-max p-0 text-xs leading-relaxed text-foreground">
            {lines.map((line, index) => (
              <div key={`${entry.path}-${index}`} className="grid grid-cols-[4rem_1fr] border-b border-white/[0.035]">
                <span className="select-none border-r border-white/[0.06] px-3 py-1 text-right font-mono text-muted-foreground/55">
                  {index + 1}
                </span>
                <code className="px-4 py-1 font-mono">{line || " "}</code>
              </div>
            ))}
          </pre>
        </div>
      )}
    </div>
  )
}

export function RepositoryExplorer({ manifest }: ExplorerProps) {
  const [query, setQuery] = useState("")
  const [selectedPath, setSelectedPath] = useState(manifest.featuredPaths[0] ?? "")
  const [activeTab, setActiveTab] = useState<ExplorerTab>("code")
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(
    () => new Set(["", "crates", "scripts", "slate-website", ...getParentPaths(manifest.featuredPaths[0] ?? "")]),
  )
  const [selectedEntry, setSelectedEntry] = useState<SelectedEntry>(null)
  const [readmeEntry, setReadmeEntry] = useState<Extract<RepositoryEntry, { type: "file" }> | null>(null)
  const [loadingEntry, setLoadingEntry] = useState(false)
  const [loadingReadme, setLoadingReadme] = useState(false)
  const [entryError, setEntryError] = useState("")

  const nodes = useMemo(() => flattenTree(manifest.tree), [manifest.tree])
  const selectedNode = findNode(manifest.tree, selectedPath) ?? manifest.tree
  const searchTerm = query.trim().toLowerCase()
  const searchResults = searchTerm
    ? nodes
        .filter((node) => {
          const haystack = `${node.path} ${node.name} ${node.language ?? ""}`.toLowerCase()
          return haystack.includes(searchTerm)
        })
        .slice(0, 80)
    : []
  const directoryReadme = selectedNode.type === "directory" ? findReadme(selectedNode) : null
  const hasReadme = Boolean(directoryReadme)

  useEffect(() => {
    if (selectedNode.type !== "file") {
      setSelectedEntry(null)
      setLoadingEntry(false)
      setEntryError("")
      return
    }

    let cancelled = false
    setLoadingEntry(true)
    setEntryError("")

    fetch(`/api/repository?path=${encodeURIComponent(selectedNode.path)}`)
      .then(async (response) => {
        if (!response.ok) throw new Error("File could not be loaded")
        return (await response.json()) as RepositoryEntry
      })
      .then((entry) => {
        if (!cancelled) setSelectedEntry(entry)
      })
      .catch((error) => {
        if (!cancelled) setEntryError(error instanceof Error ? error.message : "File could not be loaded")
      })
      .finally(() => {
        if (!cancelled) setLoadingEntry(false)
      })

    return () => {
      cancelled = true
    }
  }, [selectedNode.path, selectedNode.type])

  useEffect(() => {
    if (!directoryReadme) {
      setReadmeEntry(null)
      if (activeTab === "readme") setActiveTab("code")
      return
    }

    let cancelled = false
    setLoadingReadme(true)

    fetch(`/api/repository?path=${encodeURIComponent(directoryReadme.path)}`)
      .then(async (response) => {
        if (!response.ok) throw new Error("README could not be loaded")
        return (await response.json()) as RepositoryEntry
      })
      .then((entry) => {
        if (!cancelled) setReadmeEntry(entry.type === "file" ? entry : null)
      })
      .catch(() => {
        if (!cancelled) setReadmeEntry(null)
      })
      .finally(() => {
        if (!cancelled) setLoadingReadme(false)
      })

    return () => {
      cancelled = true
    }
  }, [activeTab, directoryReadme])

  const selectNode = (node: RepositoryNode) => {
    setSelectedPath(node.path)
    setExpandedPaths((current) => {
      const next = new Set(current)
      for (const parent of getParentPaths(node.path)) next.add(parent)
      if (node.type === "directory") next.add(node.path)
      return next
    })
  }

  const togglePath = (pathToToggle: string) => {
    setExpandedPaths((current) => {
      const next = new Set(current)
      if (next.has(pathToToggle)) next.delete(pathToToggle)
      else next.add(pathToToggle)
      return next
    })
  }

  const selectPath = (pathToSelect: string) => {
    const nextNode = findNode(manifest.tree, pathToSelect)
    if (nextNode) selectNode(nextNode)
  }

  return (
    <div className="mx-auto max-w-[1440px] px-4 pb-24 pt-24 md:px-6 lg:px-8">
      <RepositoryHeader manifest={manifest} />

      <div className="mt-5">
        <Toolbar query={query} searchCount={searchResults.length} onQueryChange={setQuery} onSelectPath={selectPath} />
      </div>

      <div className="mt-5">
        <RepositoryTabs activeTab={activeTab} onTabChange={setActiveTab} hasReadme={hasReadme} />
      </div>

      <section className="mt-5 grid gap-5 xl:grid-cols-[320px_1fr]">
        <aside className="xl:sticky xl:top-24 xl:h-[calc(100vh-7rem)]">
          <div className="overflow-hidden border border-border bg-background">
            <div className="flex items-center justify-between border-b border-border bg-secondary/20 px-3 py-2.5">
              <div className="font-mono text-xs text-foreground">Files</div>
              <div className="font-mono text-[11px] text-muted-foreground">{formatNumber(manifest.stats.files)}</div>
            </div>
            <div className="max-h-[42vh] overflow-auto p-2 xl:max-h-[calc(100vh-10rem)]">
              {searchTerm ? (
                <div className="space-y-1">
                  {searchResults.map((node) => (
                    <button
                      key={node.path}
                      type="button"
                      onClick={() => selectNode(node)}
                      className="flex w-full items-center gap-3 px-2 py-2 text-left transition-colors hover:bg-secondary/40"
                    >
                      {getFileIcon(node)}
                      <span className="min-w-0 flex-1">
                        <span className="block truncate font-mono text-xs text-foreground">{node.name}</span>
                        <span className="block truncate font-mono text-[10px] text-muted-foreground">{node.path || "root"}</span>
                      </span>
                    </button>
                  ))}
                </div>
              ) : (
                <TreeNode
                  node={manifest.tree}
                  depth={0}
                  selectedPath={selectedPath}
                  expandedPaths={expandedPaths}
                  onSelect={selectNode}
                  onToggle={togglePath}
                />
              )}
            </div>
          </div>
        </aside>

        <section className="min-w-0 space-y-4">
          <div className="flex flex-col gap-3 border border-border bg-background p-4 lg:flex-row lg:items-center lg:justify-between">
            <Breadcrumbs path={selectedNode.path} onSelectPath={selectPath} />
            <div className="flex shrink-0 flex-wrap gap-2 font-mono text-[11px] text-muted-foreground">
              <span className="border border-border px-2 py-1">{selectedNode.type}</span>
              <span className="border border-border px-2 py-1">{formatBytes(selectedNode.size)}</span>
              {selectedNode.type === "file" && selectedNode.language ? (
                <span className={`border px-2 py-1 ${getLanguageTone(selectedNode.language)}`}>{selectedNode.language}</span>
              ) : null}
            </div>
          </div>

          {activeTab === "insights" ? (
            <InsightsPanel manifest={manifest} />
          ) : activeTab === "readme" ? (
            loadingReadme ? (
              <div className="border border-border bg-background p-8">
                <div className="flex items-center gap-3 font-mono text-sm text-muted-foreground">
                  <TerminalSquare className="h-4 w-4 animate-pulse" />
                  Loading README...
                </div>
              </div>
            ) : (
              <ReadmePanel entry={readmeEntry} />
            )
          ) : (
            <div className="grid gap-5 2xl:grid-cols-[1fr_300px]">
              <div className="min-w-0">
                {selectedNode.type === "directory" ? (
                  <DirectoryPanel node={selectedNode} onSelect={selectNode} />
                ) : loadingEntry ? (
                  <div className="border border-border bg-background p-8">
                    <div className="flex items-center gap-3 font-mono text-sm text-muted-foreground">
                      <TerminalSquare className="h-4 w-4 animate-pulse" />
                      Loading source preview...
                    </div>
                  </div>
                ) : entryError ? (
                  <div className="border border-red-300/40 bg-background p-8 font-mono text-sm text-red-200">{entryError}</div>
                ) : (
                  <FilePanel entry={selectedEntry?.type === "file" ? selectedEntry : null} />
                )}
              </div>
              <DetailsPanel
                node={selectedNode}
                entry={selectedEntry}
                manifest={manifest}
                onSelectPath={selectPath}
              />
            </div>
          )}

          <div className="border border-border bg-background p-4">
            <div className="mb-3 font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground">
              Quick entry points
            </div>
            <div className="flex flex-wrap gap-2">
              {manifest.featuredPaths.map((pathName) => (
                <button
                  key={pathName}
                  type="button"
                  onClick={() => selectPath(pathName)}
                  className="border border-border px-3 py-2 font-mono text-xs text-muted-foreground transition-colors hover:border-foreground hover:text-foreground"
                >
                  {pathName}
                </button>
              ))}
            </div>
          </div>
        </section>
      </section>
    </div>
  )
}
