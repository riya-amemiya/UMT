// Bundle every compiled benchmark in module/tests/benchmark into a single
// Node/Bun compatible ESM file under benchmark-bundles. Bundling resolves
// CommonJS interop (e.g. lodash) once so the same bundle can be executed by
// any runtime in the benchmark matrix.
import { spawnSync } from "node:child_process";
import { mkdirSync, readdirSync, rmSync } from "node:fs";
import { join } from "node:path";

const IN_DIR = process.env.BENCH_IN_DIR ?? "module/tests/benchmark";
const OUT_DIR = process.env.BENCH_OUT_DIR ?? "benchmark-bundles";
const FILTER = process.env.BENCH_FILTER;

rmSync(OUT_DIR, { recursive: true, force: true });
mkdirSync(OUT_DIR, { recursive: true });

const files = readdirSync(IN_DIR)
  .filter((f) => f.endsWith(".js"))
  .filter((f) => !FILTER || new RegExp(FILTER).test(f))
  .sort();

console.error(`Bundling ${files.length} benchmark file(s) into ${OUT_DIR}...`);

let failed = 0;
for (let i = 0; i < files.length; i++) {
  const f = files[i];
  const inPath = `./${join(IN_DIR, f)}`;
  const outPath = join(OUT_DIR, f.replace(/\.js$/, ".mjs"));
  const proc = spawnSync(
    "bun",
    [
      "build",
      inPath,
      "--target=node",
      "--format=esm",
      `--outfile=${outPath}`,
    ],
    { encoding: "utf-8" },
  );
  if (proc.status !== 0) {
    failed++;
    console.error(`[${i + 1}/${files.length}] FAIL ${f}`);
    console.error(proc.stderr);
  } else {
    console.error(`[${i + 1}/${files.length}] OK   ${f}`);
  }
}

if (failed > 0) {
  console.error(`Bundling failed for ${failed} file(s).`);
  process.exit(1);
}
console.error(`Bundled ${files.length} file(s) into ${OUT_DIR}.`);
