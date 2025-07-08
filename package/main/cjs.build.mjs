import { $, write } from "bun";

const packageJson = await $`cat ./package.json`.json();
packageJson.name = "umt-common";
packageJson.type = "commonjs";

await write("common-module/package.json", JSON.stringify(packageJson, null, 2));

console.log(
  "Changed package.json name to 'umt-common' and copied to common-module.",
);
