import { readdir, readFile, writeFile, stat } from "node:fs/promises";
import { join, dirname } from "node:path";

const moduleDir = "./module";

async function getAllJsFiles(dir) {
  const files = [];
  const entries = await readdir(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    if (entry.isDirectory()) {
      // Skip tests directory
      if (entry.name === "tests") continue;
      files.push(...(await getAllJsFiles(fullPath)));
    } else if (entry.name.endsWith(".js")) {
      files.push(fullPath);
    }
  }

  return files;
}

async function pathExists(path) {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}

async function fixImports(filePath) {
  let content = await readFile(filePath, "utf-8");
  const fileDir = dirname(filePath);

  // Match import/export statements with relative paths that don't have extensions
  // e.g., export * from "./Advance" or import { foo } from "./bar"
  const importRegex = /(from\s+["'])(\.[^"']+)(["'])/g;

  let modified = false;
  const newContent = content.replace(importRegex, (match, prefix, importPath, suffix) => {
    // Skip if already has .js extension
    if (importPath.endsWith(".js")) {
      return match;
    }

    // Check if it's a directory import (path resolves to a directory with index.js)
    const fullPath = join(fileDir, importPath);
    const indexPath = join(fullPath, "index.js");
    const directPath = fullPath + ".js";

    // We'll check synchronously by trying both paths
    // For ESM compatibility, we need to add /index.js for directory imports
    // or .js for file imports

    // Since we can't do async in replace, we'll use a simple heuristic:
    // If the path doesn't have an extension, try adding /index.js first
    modified = true;
    return `${prefix}${importPath}/index.js${suffix}`;
  });

  if (modified) {
    await writeFile(filePath, newContent);
    console.log(`Fixed imports in: ${filePath}`);
  }
}

async function fixImportsWithCheck(filePath) {
  let content = await readFile(filePath, "utf-8");
  const fileDir = dirname(filePath);

  // Match import/export statements with relative paths that don't have extensions
  const importRegex = /(from\s+["'])(\.[^"']+)(["'])/g;

  let modified = false;
  const matches = [...content.matchAll(importRegex)];

  for (const match of matches) {
    const [fullMatch, prefix, importPath, suffix] = match;

    // Skip if already has .js extension
    if (importPath.endsWith(".js")) {
      continue;
    }

    const fullPath = join(fileDir, importPath);
    const indexPath = join(fullPath, "index.js");
    const directPath = fullPath + ".js";

    // Check if it's a directory import or file import
    const isDirectory = await pathExists(indexPath);
    const isFile = await pathExists(directPath);

    let newImportPath;
    if (isDirectory) {
      newImportPath = `${importPath}/index.js`;
    } else if (isFile) {
      newImportPath = `${importPath}.js`;
    } else {
      console.warn(`Warning: Could not resolve import "${importPath}" in ${filePath}`);
      continue;
    }

    content = content.replace(fullMatch, `${prefix}${newImportPath}${suffix}`);
    modified = true;
  }

  if (modified) {
    await writeFile(filePath, content);
    console.log(`Fixed imports in: ${filePath}`);
  }
}

async function main() {
  console.log("Fixing ESM imports in module directory...");
  const files = await getAllJsFiles(moduleDir);
  console.log(`Found ${files.length} JavaScript files`);

  for (const file of files) {
    await fixImportsWithCheck(file);
  }

  console.log("Done!");
}

main().catch(console.error);
