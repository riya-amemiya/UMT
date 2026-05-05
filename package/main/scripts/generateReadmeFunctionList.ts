/**
 * Regenerates the "Function List" section of README.md from typedoc output.
 *
 * Walks the public export tree under src/<Module>/ and collects every
 * top-level exported symbol per module. Each symbol is matched against
 * typedoc's generated doc files (doc/<Kind>.<Name>.md) to determine the
 * GitHub Wiki page produced by typedoc-github-wiki-theme. The README's
 * Function List section is rewritten with one section per module and
 * each entry linking to its wiki page.
 */

import { execSync } from "node:child_process";
import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, resolve } from "node:path";

const PACKAGE_ROOT = resolve(import.meta.dir, "..");
const SRC_DIR = join(PACKAGE_ROOT, "src");
const DOC_DIR = join(PACKAGE_ROOT, "doc");
const README_PATH = join(PACKAGE_ROOT, "README.md");

const WIKI_BASE_URL = "https://github.com/riya-amemiya/UMT/wiki";

// Modules excluded from the Function List (e.g. internal type definitions).
const EXCLUDED_MODULES = new Set(["types", "tests"]);

const KIND_PRIORITY = [
  "Function",
  "Class",
  "Variable",
  "Interface",
  "TypeAlias",
  "Enum",
];

type DocIndex = Map<string, string>;

const buildDocIndex = (): DocIndex => {
  const index: DocIndex = new Map();
  if (!existsSync(DOC_DIR)) {
    return index;
  }
  for (const entry of readdirSync(DOC_DIR)) {
    if (!entry.endsWith(".md")) {
      continue;
    }
    const base = entry.slice(0, -3);
    const dotIndex = base.indexOf(".");
    if (dotIndex <= 0) {
      continue;
    }
    const kind = base.slice(0, dotIndex);
    const name = base.slice(dotIndex + 1);
    if (!KIND_PRIORITY.includes(kind)) {
      continue;
    }
    const existing = index.get(name);
    if (
      !existing ||
      KIND_PRIORITY.indexOf(kind) < KIND_PRIORITY.indexOf(existing)
    ) {
      index.set(name, kind);
    }
  }
  return index;
};

const EXPORT_STAR_RE = /export\s+\*\s+from\s+["']([^"']+)["']/g;
const EXPORT_NAMED_FROM_RE = /export\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']/g;
const EXPORT_NAMED_LOCAL_RE = /export\s*\{([^}]+)\}\s*;?(?!\s*from)/g;
const EXPORT_DECL_RE =
  /^export\s+(?:declare\s+)?(?:async\s+)?(?:abstract\s+)?(?:const|let|var|function\*?|class|interface|type|enum)\s+([A-Za-z_$][\w$]*)/gm;

const resolveImport = (fromFile: string, spec: string): string | null => {
  const baseDir = dirname(fromFile);
  const target = resolve(baseDir, spec);
  if (existsSync(target) && statSync(target).isDirectory()) {
    const indexPath = join(target, "index.ts");
    return existsSync(indexPath) ? indexPath : null;
  }
  if (existsSync(`${target}.ts`)) {
    return `${target}.ts`;
  }
  if (existsSync(`${target}.tsx`)) {
    return `${target}.tsx`;
  }
  return null;
};

const collectExportsFromFile = (
  filePath: string,
  visited: Set<string>,
): string[] => {
  if (visited.has(filePath)) {
    return [];
  }
  visited.add(filePath);

  const source = readFileSync(filePath, "utf-8");
  const names = new Set<string>();

  for (const match of source.matchAll(EXPORT_STAR_RE)) {
    const next = resolveImport(filePath, match[1]);
    if (next) {
      for (const name of collectExportsFromFile(next, visited)) {
        names.add(name);
      }
    }
  }

  const parseSpecifiers = (raw: string): string[] =>
    raw
      .split(",")
      .map((part) => part.trim())
      .filter(Boolean)
      .map((part) => {
        // Handle "Foo as Bar" -> "Bar".
        const asMatch = part.match(/\s+as\s+([A-Za-z_$][\w$]*)/);
        if (asMatch) {
          return asMatch[1];
        }
        // Drop "type" keyword prefix (`export { type Foo } ...`).
        return part.replace(/^type\s+/, "").trim();
      })
      .filter((name) => /^[A-Za-z_$][\w$]*$/.test(name));

  for (const match of source.matchAll(EXPORT_NAMED_FROM_RE)) {
    for (const name of parseSpecifiers(match[1])) {
      names.add(name);
    }
  }

  // Strip `export { ... } from "..."` matches before scanning local re-exports
  // so the same block is not double-counted.
  const sourceWithoutFromExports = source.replace(EXPORT_NAMED_FROM_RE, "");
  for (const match of sourceWithoutFromExports.matchAll(
    EXPORT_NAMED_LOCAL_RE,
  )) {
    for (const name of parseSpecifiers(match[1])) {
      names.add(name);
    }
  }

  for (const match of source.matchAll(EXPORT_DECL_RE)) {
    names.add(match[1]);
  }

  return [...names];
};

const collectModuleExports = (moduleDir: string): string[] => {
  const indexPath = join(moduleDir, "index.ts");
  if (!existsSync(indexPath)) {
    return [];
  }
  return collectExportsFromFile(indexPath, new Set());
};

const formatSection = (
  module: string,
  names: string[],
  docIndex: DocIndex,
): string => {
  const lines = [`### ${module}`, ""];
  const sorted = [...names].sort((a, b) =>
    a.localeCompare(b, "en", { sensitivity: "base" }),
  );
  for (const name of sorted) {
    const kind = docIndex.get(name);
    if (!kind) {
      // Skip names that typedoc did not document (internal helpers, types
      // intentionally excluded from the API surface, etc.).
      continue;
    }
    const url = `${WIKI_BASE_URL}/${kind}.${encodeURIComponent(name)}`;
    lines.push(`- [${name}](${url})`);
  }
  lines.push("");
  return lines.join("\n");
};

const generateFunctionList = (docIndex: DocIndex): string => {
  const moduleDirs = readdirSync(SRC_DIR)
    .filter((entry) => {
      if (EXCLUDED_MODULES.has(entry)) {
        return false;
      }
      const path = join(SRC_DIR, entry);
      return statSync(path).isDirectory();
    })
    .sort((a, b) => a.localeCompare(b, "en", { sensitivity: "base" }));

  const sections: string[] = ["## Function List", ""];
  for (const module of moduleDirs) {
    const names = collectModuleExports(join(SRC_DIR, module));
    if (names.length === 0) {
      continue;
    }
    const section = formatSection(module, names, docIndex);
    if (section.split("\n").some((line) => line.startsWith("- "))) {
      sections.push(section);
    }
  }
  return `${sections.join("\n").trimEnd()}\n`;
};

const replaceFunctionList = (readme: string, generated: string): string => {
  const marker = "## Function List";
  const idx = readme.indexOf(marker);
  if (idx === -1) {
    return `${readme.trimEnd()}\n\n${generated}`;
  }
  return `${readme.slice(0, idx).trimEnd()}\n\n${generated}`;
};

const main = (): void => {
  console.log("Running typedoc to refresh doc/...");
  execSync("bunx typedoc", {
    cwd: PACKAGE_ROOT,
    stdio: "inherit",
  });

  const docIndex = buildDocIndex();
  if (docIndex.size === 0) {
    throw new Error(
      `No typedoc output found at ${DOC_DIR}; aborting README generation.`,
    );
  }

  const generated = generateFunctionList(docIndex);
  const readme = readFileSync(README_PATH, "utf-8");
  const updated = replaceFunctionList(readme, generated);
  writeFileSync(README_PATH, updated);
  console.log(`Updated ${README_PATH}`);
};

main();
