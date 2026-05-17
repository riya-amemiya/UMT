// Run every compiled benchmark file under module/tests/benchmark with the
// given runtime (bun or node), capture mitata's JSON output, and aggregate the
// results into a single JSON file.
//
// Usage: <runtime> bun scripts/benchmark/runAll.mjs <cmd> <out.json>
//   cmd: command to execute each benchmark (e.g. "bun", "node")
//   out: path of aggregated JSON result file
import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";

const BENCH_DIR = process.env.BENCH_DIR ?? "benchmark-bundles";
const CMD = process.argv[2];
const OUT = process.argv[3];
const TIMEOUT_MS = Number(process.env.BENCH_TIMEOUT_MS ?? 20 * 60 * 1000);
const FILTER = process.env.BENCH_FILTER;

if (!CMD || !OUT) {
  console.error("usage: runAll.mjs <cmd> <out.json>");
  process.exit(2);
}

// Patch each bundled benchmark so mitata emits machine-readable JSON
// instead of its colored TTY table. The original benchmark sources call
// `await run()` with no arguments; replace those call sites in the bundled
// output only so the source tree stays untouched.
const RUN_OPTS =
  "{colors:false,format:{json:{debug:false,samples:false}}}";
const RUN_RE = /\bawait\s+run\s*\(\s*\)/g;

const files = readdirSync(BENCH_DIR)
  .filter((f) => f.endsWith(".mjs") || f.endsWith(".js"))
  .filter((f) => !FILTER || new RegExp(FILTER).test(f))
  .sort();

let patchedCount = 0;
for (const f of files) {
  const p = join(BENCH_DIR, f);
  const before = readFileSync(p, "utf-8");
  const after = before.replace(RUN_RE, `await run(${RUN_OPTS})`);
  if (after !== before) {
    writeFileSync(p, after);
    patchedCount++;
  }
}
console.error(
  `Patched ${patchedCount}/${files.length} benchmark file(s) for JSON output.`,
);

const results = [];
for (let i = 0; i < files.length; i++) {
  const file = files[i];
  const path = join(BENCH_DIR, file);
  const label = `[${i + 1}/${files.length}] ${file}`;
  process.stderr.write(`${label} ... `);

  const startedAt = Date.now();
  const proc = spawnSync(CMD, [path], {
    encoding: "utf-8",
    maxBuffer: 512 * 1024 * 1024,
    timeout: TIMEOUT_MS,
    env: { ...process.env, NO_COLOR: "1", FORCE_COLOR: "0" },
  });
  const elapsedMs = Date.now() - startedAt;

  const stdout = proc.stdout ?? "";
  const stderr = proc.stderr ?? "";

  // mitata's json format writes a single JSON object to stdout. Some
  // runtimes emit deprecation warnings before it, so find the last
  // top-level object boundary instead of trusting position 0.
  let parsed = null;
  let parseError = null;
  const start = stdout.indexOf("{");
  const end = stdout.lastIndexOf("}");
  if (start === -1 || end === -1 || end < start) {
    parseError = "no JSON object found in stdout";
  } else {
    try {
      parsed = JSON.parse(stdout.slice(start, end + 1));
    } catch (e) {
      parseError = `JSON parse failed: ${String(e)}`;
    }
  }

  process.stderr.write(
    `${(elapsedMs / 1000).toFixed(1)}s exit=${proc.status}` +
      (parseError ? ` (parse error: ${parseError})` : "") +
      "\n",
  );

  results.push({
    file: basename(file),
    elapsedMs,
    exitCode: proc.status ?? null,
    signal: proc.signal ?? null,
    parseError,
    data: parsed,
    stderrTail: stderr.length > 4000 ? stderr.slice(-4000) : stderr,
  });
}

writeFileSync(
  OUT,
  JSON.stringify(
    {
      cmd: CMD,
      filter: FILTER ?? null,
      generatedAt: new Date().toISOString(),
      files: results,
    },
    null,
    2,
  ),
);
console.error(`Wrote ${OUT}`);
