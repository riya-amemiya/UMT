// Bundle every compiled benchmark in module/tests/benchmark into a single
// Node/Bun compatible ESM file under benchmark-bundles. Bundling resolves
// CommonJS interop (e.g. lodash) once so the same bundle can be executed by
// any runtime in the benchmark matrix.
// biome-ignore lint/correctness/noNodejsModules: readdirSync/mkdirSync/rmSync have no Bun-native equivalent
import { mkdirSync, readdirSync, rmSync } from "node:fs";
// biome-ignore lint/correctness/noNodejsModules: node:path has no Bun-native equivalent
import { join } from "node:path";
// biome-ignore lint/correctness/noNodejsModules: process.exit has no Bun-native equivalent
import process from "node:process";

const IN_DIR = Bun.env.BENCH_IN_DIR ?? "module/tests/benchmark";
const OUT_DIR = Bun.env.BENCH_OUT_DIR ?? "benchmark-bundles";
const FILTER = Bun.env.BENCH_FILTER;

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
  const proc = Bun.spawnSync({
    cmd: [
      "bun",
      "build",
      inPath,
      "--target=node",
      "--format=esm",
      `--outfile=${outPath}`,
    ],
    stdout: "pipe",
    stderr: "pipe",
  });
  if (proc.exitCode === 0) {
    console.error(`[${i + 1}/${files.length}] OK   ${f}`);
  } else {
    failed++;
    console.error(`[${i + 1}/${files.length}] FAIL ${f}`);
    console.error(proc.stderr.toString());
  }
}

if (failed > 0) {
  console.error(`Bundling failed for ${failed} file(s).`);
  process.exit(1);
}
console.error(`Bundled ${files.length} file(s) into ${OUT_DIR}.`);
