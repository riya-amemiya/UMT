import { Glob } from "bun";
import { mkdir, cp } from "node:fs/promises";
import { dirname, join } from "node:path";

// Copy .d.ts files from module/ to module/es5/ (excluding tests and es5 itself)
const glob = new Glob("**/*.d.ts");
for await (const file of glob.scan({ cwd: "./module", absolute: false })) {
  // Skip tests directory and es5 directory
  if (file.startsWith("tests/") || file.includes("/tests/") || file.startsWith("es5/") || file.includes("/es5/")) {
    continue;
  }
  const srcPath = join("./module", file);
  const destPath = join("./module/es5", file);
  await mkdir(dirname(destPath), { recursive: true });
  await cp(srcPath, destPath);
}
console.log("Copied .d.ts files from module/ to module/es5/");
