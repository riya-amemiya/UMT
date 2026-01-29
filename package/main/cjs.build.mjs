import { $, write, Glob } from "bun";
import { mkdir, cp } from "node:fs/promises";
import { dirname, join } from "node:path";

// Copy .d.ts files from module/ to common-module/module/ (excluding tests)
const glob = new Glob("**/*.d.ts");
for await (const file of glob.scan({ cwd: "./module", absolute: false })) {
  // Skip tests directory
  if (file.startsWith("tests/") || file.includes("/tests/")) {
    continue;
  }
  const srcPath = join("./module", file);
  const destPath = join("./common-module/module", file);
  await mkdir(dirname(destPath), { recursive: true });
  await cp(srcPath, destPath);
}
console.log("Copied .d.ts files from module/ to common-module/module/");

const packageJson = await $`cat ./package.json`.json();
packageJson.name = "umt-common";
packageJson.type = "commonjs";

// Convert exports from ESM (import) to CommonJS (require)
if (packageJson.exports) {
  for (const key of Object.keys(packageJson.exports)) {
    const { import: importPath, ...rest } = packageJson.exports[key];
    if (importPath) {
      packageJson.exports[key] = { ...rest, require: importPath };
    }
  }
}

// Remove module field (ESM-only)
const { module: _, ...restPackageJson } = packageJson;
Object.assign(packageJson, restPackageJson);
packageJson.module = undefined;

await write("common-module/package.json", JSON.stringify(packageJson, null, 2));

console.log(
  "Changed package.json name to 'umt-common' and copied to common-module.",
);
