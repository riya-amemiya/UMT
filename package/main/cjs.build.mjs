import { $, write } from "bun";

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
