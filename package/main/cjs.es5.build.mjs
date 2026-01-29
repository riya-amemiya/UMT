import { Glob } from "bun";
import { mkdir, cp } from "node:fs/promises";
import { dirname, join } from "node:path";

// Copy .d.ts files from module/es5/ to common-module/module/es5/ (excluding tests)
const glob = new Glob("**/*.d.ts");
for await (const file of glob.scan({ cwd: "./module/es5", absolute: false })) {
  // Skip tests directory
  if (file.startsWith("tests/") || file.includes("/tests/")) {
    continue;
  }
  const srcPath = join("./module/es5", file);
  const destPath = join("./common-module/module/es5", file);
  await mkdir(dirname(destPath), { recursive: true });
  await cp(srcPath, destPath);
}
console.log("Copied .d.ts files from module/es5/ to common-module/module/es5/");
