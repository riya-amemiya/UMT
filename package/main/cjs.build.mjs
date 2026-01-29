import { $, write } from "bun";

const packageJson = await $`cat ./package.json`.json();
packageJson.name = "umt-common";
packageJson.type = "commonjs";

// Convert exports from ESM (import) to CommonJS (require)
if (packageJson.exports) {
  for (const key of Object.keys(packageJson.exports)) {
    const entry = packageJson.exports[key];
    if (entry.import) {
      entry.require = entry.import;
      delete entry.import;
    }
  }
}

// Remove module field (ESM-only)
delete packageJson.module;

await write("common-module/package.json", JSON.stringify(packageJson, null, 2));

console.log(
  "Changed package.json name to 'umt-common' and copied to common-module.",
);
