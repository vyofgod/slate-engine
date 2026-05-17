import fs from "fs/promises"
import path from "path"
import { execFile } from "child_process"
import { promisify } from "util"

export type RepositoryNodeType = "directory" | "file"

export type RepositoryNode = {
  name: string
  path: string
  type: RepositoryNodeType
  size: number
  children?: RepositoryNode[]
  extension?: string
  language?: string
  lines?: number
  isBinary?: boolean
}

export type LanguageStat = {
  language: string
  files: number
  lines: number
  bytes: number
}

export type CommitSummary = {
  hash: string
  subject: string
  author: string
  date: string
}

export type RepositoryManifest = {
  rootName: string
  generatedAt: string
  tree: RepositoryNode
  stats: {
    files: number
    directories: number
    textFiles: number
    binaryFiles: number
    totalBytes: number
    totalLines: number
    languages: LanguageStat[]
  }
  recentCommits: CommitSummary[]
  featuredPaths: string[]
  excluded: string[]
}

export type RepositoryEntry =
  | {
      type: "directory"
      path: string
      name: string
      size: number
      children: RepositoryNode[]
    }
  | {
      type: "file"
      path: string
      name: string
      size: number
      language: string
      extension: string
      lines: number
      isBinary: boolean
      content: string | null
      truncated: boolean
    }

const REPO_ROOT = path.join(/*turbopackIgnore: true*/ process.cwd(), "..", "..")
const MAX_ANALYZE_BYTES = 1024 * 1024
const MAX_CONTENT_BYTES = 240 * 1024
const execFileAsync = promisify(execFile)

const EXCLUDED_SEGMENTS = new Set([
  ".git",
  ".next",
  ".turbo",
  ".cache",
  ".parcel-cache",
  ".agents",
  ".codex",
  "target",
  "node_modules",
  "dist",
  "build",
  "coverage",
  "__pycache__",
  "out",
  "output",
])

const EXCLUDED_LABELS = [
  ".git",
  "target",
  "node_modules",
  ".next",
  ".agents",
  ".codex",
  "out",
  "output",
  "coverage",
  "dist/build caches",
  "__pycache__",
]

const TEXT_EXTENSIONS = new Set([
  ".css",
  ".html",
  ".js",
  ".json",
  ".lock",
  ".md",
  ".mjs",
  ".rs",
  ".sh",
  ".svg",
  ".toml",
  ".ts",
  ".tsx",
  ".txt",
  ".wgsl",
  ".yaml",
  ".yml",
])

const BINARY_EXTENSIONS = new Set([
  ".gif",
  ".ico",
  ".jpeg",
  ".jpg",
  ".otf",
  ".pdf",
  ".png",
  ".ttf",
  ".webp",
  ".woff",
  ".woff2",
])

const LANGUAGE_BY_EXTENSION: Record<string, string> = {
  ".css": "CSS",
  ".html": "HTML",
  ".js": "JavaScript",
  ".json": "JSON",
  ".lock": "Lockfile",
  ".md": "Markdown",
  ".mjs": "JavaScript",
  ".rs": "Rust",
  ".sh": "Shell",
  ".svg": "SVG",
  ".toml": "TOML",
  ".ts": "TypeScript",
  ".tsx": "TypeScript React",
  ".txt": "Text",
  ".wgsl": "WGSL",
  ".yaml": "YAML",
  ".yml": "YAML",
}

function toRepositoryPath(absolutePath: string) {
  const relativePath = path.relative(REPO_ROOT, absolutePath)
  if (!relativePath) return ""
  return relativePath.split(path.sep).join("/")
}

function normalizeRepositoryPath(input: string) {
  const unixPath = input.replace(/\\/g, "/")
  const normalized = path.posix.normalize(unixPath).replace(/^\/+/, "")
  if (normalized === ".") return ""
  if (normalized.startsWith("../") || normalized === "..") {
    throw new Error("Path escapes repository root")
  }
  return normalized
}

function resolveRepositoryPath(input: string) {
  const relativePath = normalizeRepositoryPath(input)
  const absolutePath = path.resolve(REPO_ROOT, relativePath)
  const relativeFromRoot = path.relative(REPO_ROOT, absolutePath)

  if (relativeFromRoot.startsWith("..") || path.isAbsolute(relativeFromRoot)) {
    throw new Error("Path escapes repository root")
  }

  return { absolutePath, relativePath }
}

function shouldExclude(relativePath: string) {
  if (!relativePath) return false
  const segments = relativePath.split("/")
  return segments.some((segment) => EXCLUDED_SEGMENTS.has(segment))
}

function getExtension(fileName: string) {
  if (fileName === "Cargo.lock") return ".lock"
  return path.extname(fileName).toLowerCase()
}

function detectLanguage(fileName: string) {
  if (fileName === ".gitignore") return "Git Ignore"
  if (fileName === "webapis") return "Web API Catalog"
  const extension = getExtension(fileName)
  return LANGUAGE_BY_EXTENSION[extension] ?? (extension ? extension.slice(1).toUpperCase() : "Plain Text")
}

function isLikelyText(fileName: string, sample: Buffer) {
  const extension = getExtension(fileName)
  if (BINARY_EXTENSIONS.has(extension)) return false
  if (TEXT_EXTENSIONS.has(extension)) return true
  return !sample.includes(0)
}

function countLines(content: string) {
  if (!content) return 0
  return content.split(/\r\n|\r|\n/).length
}

async function analyzeFile(absolutePath: string, fileName: string, size: number) {
  const extension = getExtension(fileName)
  const language = detectLanguage(fileName)
  const sample = await fs.readFile(absolutePath)
  const isBinary = !isLikelyText(fileName, sample.subarray(0, Math.min(sample.length, 4096)))
  const shouldAnalyzeText = !isBinary && size <= MAX_ANALYZE_BYTES
  const content = shouldAnalyzeText ? sample.toString("utf8") : ""

  return {
    extension,
    language,
    isBinary,
    lines: shouldAnalyzeText ? countLines(content) : 0,
  }
}

function sortNodes(nodes: RepositoryNode[]) {
  return nodes.sort((left, right) => {
    if (left.type !== right.type) return left.type === "directory" ? -1 : 1
    return left.name.localeCompare(right.name)
  })
}

async function buildNode(absolutePath: string): Promise<RepositoryNode | null> {
  const relativePath = toRepositoryPath(absolutePath)
  if (shouldExclude(relativePath)) return null

  const stat = await fs.stat(absolutePath)
  const name = relativePath ? path.basename(absolutePath) : "slate-engine"

  if (stat.isDirectory()) {
    const entries = await fs.readdir(absolutePath, { withFileTypes: true })
    const children: RepositoryNode[] = []

    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue
      const childPath = path.join(absolutePath, entry.name)
      const childNode = await buildNode(childPath)
      if (childNode) children.push(childNode)
    }

    const sortedChildren = sortNodes(children)

    return {
      name,
      path: relativePath,
      type: "directory",
      size: sortedChildren.reduce((total, child) => total + child.size, 0),
      children: sortedChildren,
    }
  }

  if (!stat.isFile()) return null

  const analysis = await analyzeFile(absolutePath, name, stat.size)

  return {
    name,
    path: relativePath,
    type: "file",
    size: stat.size,
    extension: analysis.extension,
    language: analysis.language,
    lines: analysis.lines,
    isBinary: analysis.isBinary,
  }
}

function summarizeTree(root: RepositoryNode) {
  const languageStats = new Map<string, LanguageStat>()
  const stats = {
    files: 0,
    directories: 0,
    textFiles: 0,
    binaryFiles: 0,
    totalBytes: 0,
    totalLines: 0,
    languages: [] as LanguageStat[],
  }

  const visit = (node: RepositoryNode, includeDirectory: boolean) => {
    if (node.type === "directory") {
      if (includeDirectory) stats.directories += 1
      for (const child of node.children ?? []) visit(child, true)
      return
    }

    stats.files += 1
    stats.totalBytes += node.size
    stats.totalLines += node.lines ?? 0

    if (node.isBinary) {
      stats.binaryFiles += 1
      return
    }

    stats.textFiles += 1
    const language = node.language ?? "Unknown"
    const current = languageStats.get(language) ?? { language, files: 0, lines: 0, bytes: 0 }
    current.files += 1
    current.lines += node.lines ?? 0
    current.bytes += node.size
    languageStats.set(language, current)
  }

  visit(root, false)
  stats.languages = Array.from(languageStats.values()).sort((left, right) => right.files - left.files)

  return stats
}

async function readRecentCommits(): Promise<CommitSummary[]> {
  try {
    const { stdout } = await execFileAsync("git", ["log", "-6", "--date=short", "--pretty=format:%h%x1f%ad%x1f%an%x1f%s"], {
      cwd: REPO_ROOT,
      timeout: 1500,
      maxBuffer: 64 * 1024,
    })

    return stdout
      .split("\n")
      .filter(Boolean)
      .map((line) => {
        const [hash, date, author, subject] = line.split("\x1f")
        return {
          hash: hash ?? "",
          date: date ?? "",
          author: author ?? "",
          subject: subject ?? "",
        }
      })
  } catch {
    return []
  }
}

export async function buildRepositoryManifest(): Promise<RepositoryManifest> {
  const tree = await buildNode(REPO_ROOT)

  if (!tree) {
    throw new Error("Repository tree could not be built")
  }

  return {
    rootName: "slate-engine",
    generatedAt: new Date().toISOString(),
    tree,
    stats: summarizeTree(tree),
    recentCommits: await readRecentCommits(),
    featuredPaths: [
      "README.md",
      "Cargo.toml",
      "crates/slate-kernel/src/lib.rs",
      "crates/slate-dispatcher/src/lib.rs",
      "crates/slate-ais/src/lib.rs",
      "scripts/browser_engine_benchmark.py",
    ],
    excluded: EXCLUDED_LABELS,
  }
}

export async function readRepositoryEntry(inputPath: string): Promise<RepositoryEntry> {
  const { absolutePath, relativePath } = resolveRepositoryPath(inputPath)

  if (shouldExclude(relativePath)) {
    throw new Error("Requested path is intentionally excluded from the explorer")
  }

  const stat = await fs.stat(absolutePath)
  const name = relativePath ? path.basename(absolutePath) : "slate-engine"

  if (stat.isDirectory()) {
    const node = await buildNode(absolutePath)
    if (!node || node.type !== "directory") throw new Error("Directory is not available")

    return {
      type: "directory",
      path: relativePath,
      name,
      size: node.size,
      children: node.children ?? [],
    }
  }

  if (!stat.isFile()) {
    throw new Error("Repository entry is not a file")
  }

  const analysis = await analyzeFile(absolutePath, name, stat.size)
  const canReadContent = !analysis.isBinary
  const rawContent = canReadContent ? await fs.readFile(absolutePath, "utf8") : null
  const truncated = Boolean(rawContent && Buffer.byteLength(rawContent, "utf8") > MAX_CONTENT_BYTES)
  const content = rawContent ? Buffer.from(rawContent).subarray(0, MAX_CONTENT_BYTES).toString("utf8") : null

  return {
    type: "file",
    path: relativePath,
    name,
    size: stat.size,
    language: analysis.language,
    extension: analysis.extension,
    lines: analysis.lines,
    isBinary: analysis.isBinary,
    content,
    truncated,
  }
}
