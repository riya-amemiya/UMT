// Compare two sets of aggregated benchmark results (base vs head) across
// multiple runtimes and emit a Markdown report suitable for a PR comment.
//
// Usage:
//   bun scripts/benchmark/compare.mjs <config.json> <out.md>
//
// config.json shape:
//   {
//     "baseSha": "...", "headSha": "...",
//     "baseRef": "main", "headRef": "feature/x",
//     "runtimes": [
//       { "id": "bun", "label": "Bun latest",
//         "base": "path/to/base/bun.json", "head": "path/to/head/bun.json" },
//       ...
//     ]
//   }
// biome-ignore lint/correctness/noNodejsModules: readFileSync/writeFileSync are needed for sync I/O
import { readFileSync, writeFileSync } from "node:fs";
// biome-ignore lint/correctness/noNodejsModules: process.exit has no Bun-native equivalent
import process from "node:process";

const [, , configPath, outPath] = Bun.argv;
if (!(configPath && outPath)) {
  console.error("usage: compare.mjs <config.json> <out.md>");
  process.exit(2);
}

const REGRESSION_THRESHOLD = Number(Bun.env.BENCH_REGRESSION_PCT ?? 5); // percent
const IMPROVEMENT_THRESHOLD = Number(Bun.env.BENCH_IMPROVEMENT_PCT ?? 5); // percent
const COMMENT_BUDGET = Number(Bun.env.BENCH_COMMENT_BUDGET ?? 60_000); // chars

const cfg = JSON.parse(readFileSync(configPath, "utf-8"));

const loadJson = (p) => {
  try {
    return JSON.parse(readFileSync(p, "utf-8"));
  } catch (e) {
    return { error: String(e), files: [] };
  }
};

// Build map: file -> bench-key -> runtime -> { base, head }
// bench-key encodes (alias, args) so different argument combos are separated.
const buildKey = (alias, args) => `${alias}::${JSON.stringify(args ?? {})}`;

const formatArgs = (args) => {
  if (!args || Object.keys(args).length === 0) {
    return "";
  }
  return Object.entries(args)
    .map(([k, v]) => `${k}=${v}`)
    .join(", ");
};

const indexResults = (resultJson) => {
  // resultJson: { files: [{ file, data: { benchmarks: [...] } }] }
  const out = new Map();
  for (const fileEntry of resultJson?.files ?? []) {
    const d = fileEntry.data;
    if (!d) {
      continue;
    }
    const benches = d.benchmarks ?? [];
    const perFile = new Map();
    for (const b of benches) {
      const alias = b.alias ?? b.name ?? "(anonymous)";
      for (const r of b.runs ?? []) {
        if (!r.stats) {
          continue;
        }
        const key = buildKey(alias, r.args);
        perFile.set(key, {
          alias,
          args: r.args ?? {},
          avg: r.stats.avg,
          min: r.stats.min,
          max: r.stats.max,
          p75: r.stats.p75,
          p99: r.stats.p99,
        });
      }
    }
    out.set(fileEntry.file, {
      meta: {
        exitCode: fileEntry.exitCode,
        parseError: fileEntry.parseError,
        elapsedMs: fileEntry.elapsedMs,
      },
      benches: perFile,
      runtime: d.context?.runtime ?? null,
      runtimeVersion: d.context?.version ?? null,
    });
  }
  return out;
};

// runtimeId -> { base: Map<file, ...>, head: Map<file, ...>, label }
const indexed = new Map();
const allFiles = new Set();
for (const rt of cfg.runtimes) {
  const baseRes = loadJson(rt.base);
  const headRes = loadJson(rt.head);
  const base = indexResults(baseRes);
  const head = indexResults(headRes);
  for (const f of base.keys()) {
    allFiles.add(f);
  }
  for (const f of head.keys()) {
    allFiles.add(f);
  }
  indexed.set(rt.id, { label: rt.label, base, head });
}

const sortedFiles = [...allFiles].sort();

// Format helpers ------------------------------------------------------------

const formatNs = (ns) => {
  if (ns == null || !Number.isFinite(ns)) {
    return "-";
  }
  if (ns < 1) {
    return `${ns.toFixed(3)} ns`;
  }
  if (ns < 1000) {
    return `${ns.toFixed(2)} ns`;
  }
  if (ns < 1_000_000) {
    return `${(ns / 1000).toFixed(2)} µs`;
  }
  if (ns < 1_000_000_000) {
    return `${(ns / 1_000_000).toFixed(2)} ms`;
  }
  return `${(ns / 1_000_000_000).toFixed(2)} s`;
};

const formatDelta = (base, head) => {
  if (base == null || head == null) {
    return { text: "-", pct: null, kind: "na" };
  }
  if (!Number.isFinite(base) || base === 0) {
    return { text: "-", pct: null, kind: "na" };
  }
  const pct = ((head - base) / base) * 100;
  const sign = pct >= 0 ? "+" : "";
  let kind = "neutral";
  if (pct >= REGRESSION_THRESHOLD) {
    kind = "regression";
  } else if (pct <= -IMPROVEMENT_THRESHOLD) {
    kind = "improvement";
  }
  return { text: `${sign}${pct.toFixed(2)}%`, pct, kind };
};

const statusIcon = (kind) =>
  kind === "regression"
    ? "🔴"
    : kind === "improvement"
      ? "🟢"
      : kind === "neutral"
        ? "⚪"
        : "·";

// Aggregate all rows --------------------------------------------------------

// rows[file][benchKey][runtimeId] = { base, head, delta }
const rows = new Map();
for (const file of sortedFiles) {
  const benchKeys = new Set();
  const perRuntime = new Map();
  for (const [rtId, { base, head }] of indexed) {
    const bFile = base.get(file);
    const hFile = head.get(file);
    perRuntime.set(rtId, { base: bFile, head: hFile });
    if (bFile) {
      for (const k of bFile.benches.keys()) {
        benchKeys.add(k);
      }
    }
    if (hFile) {
      for (const k of hFile.benches.keys()) {
        benchKeys.add(k);
      }
    }
  }
  const perBench = new Map();
  for (const k of [...benchKeys].sort()) {
    const perRt = new Map();
    let exampleAlias = k;
    let exampleArgs = {};
    for (const [rtId, { base, head }] of perRuntime) {
      const b = base?.benches.get(k);
      const h = head?.benches.get(k);
      if (b) {
        exampleAlias = b.alias;
        exampleArgs = b.args;
      } else if (h) {
        exampleAlias = h.alias;
        exampleArgs = h.args;
      }
      const baseAvg = b?.avg ?? null;
      const headAvg = h?.avg ?? null;
      perRt.set(rtId, {
        base: baseAvg,
        head: headAvg,
        delta: formatDelta(baseAvg, headAvg),
      });
    }
    perBench.set(k, {
      alias: exampleAlias,
      args: exampleArgs,
      perRuntime: perRt,
    });
  }
  rows.set(file, perBench);
}

// Build regression / improvement summary ------------------------------------
const flagged = [];
for (const [file, byBench] of rows) {
  for (const [, { alias, args, perRuntime }] of byBench) {
    for (const [rtId, entry] of perRuntime) {
      if (
        entry.delta.kind === "regression" ||
        entry.delta.kind === "improvement"
      ) {
        flagged.push({
          file,
          alias,
          args,
          runtimeId: rtId,
          runtimeLabel: indexed.get(rtId)?.label ?? rtId,
          base: entry.base,
          head: entry.head,
          delta: entry.delta,
        });
      }
    }
  }
}
flagged.sort((a, b) => Math.abs(b.delta.pct ?? 0) - Math.abs(a.delta.pct ?? 0));

// Build markdown ------------------------------------------------------------
const out = [];
out.push("## 🏎️ Benchmark Report");
out.push("");
const baseHeadLine = [
  cfg.baseRef
    ? `**base** \`${cfg.baseRef}\` @ \`${(cfg.baseSha ?? "").slice(0, 7)}\``
    : null,
  cfg.headRef
    ? `**head** \`${cfg.headRef}\` @ \`${(cfg.headSha ?? "").slice(0, 7)}\``
    : null,
]
  .filter(Boolean)
  .join(" → ");
if (baseHeadLine) {
  out.push(baseHeadLine);
}
out.push("");
out.push(
  `Threshold: regression \`+${REGRESSION_THRESHOLD}%\` / improvement \`-${IMPROVEMENT_THRESHOLD}%\` (based on \`avg\` ns/iter).`,
);
out.push("");

// Runtime versions
const versionLines = [];
for (const rt of cfg.runtimes) {
  const baseSample = [...(indexed.get(rt.id)?.base.values() ?? [])][0];
  const headSample = [...(indexed.get(rt.id)?.head.values() ?? [])][0];
  const version =
    headSample?.runtimeVersion ?? baseSample?.runtimeVersion ?? "?";
  versionLines.push(`- ${rt.label}: \`${version}\``);
}
if (versionLines.length > 0) {
  out.push("<details><summary>Runtime versions</summary>");
  out.push("");
  out.push(...versionLines);
  out.push("");
  out.push("</details>");
  out.push("");
}

// Summary section
out.push("### Summary");
out.push("");
if (flagged.length === 0) {
  out.push(
    `No benchmarks crossed the ±${REGRESSION_THRESHOLD}% threshold across any runtime.`,
  );
} else {
  const regressions = flagged.filter((f) => f.delta.kind === "regression");
  const improvements = flagged.filter((f) => f.delta.kind === "improvement");
  out.push(
    `Regressions: **${regressions.length}**, Improvements: **${improvements.length}**.`,
  );
  out.push("");
  out.push("| File | Benchmark | Runtime | Base | Head | Δ |");
  out.push("|---|---|---|---:|---:|---:|");
  const topRows = flagged.slice(0, 30);
  for (const f of topRows) {
    const argsStr = formatArgs(f.args);
    const benchLabel = argsStr ? `${f.alias} \`${argsStr}\`` : f.alias;
    out.push(
      `| \`${f.file}\` | ${benchLabel} | ${f.runtimeLabel} | ${formatNs(f.base)} | ${formatNs(f.head)} | ${statusIcon(f.delta.kind)} ${f.delta.text} |`,
    );
  }
  if (flagged.length > topRows.length) {
    out.push("");
    out.push(`_… and ${flagged.length - topRows.length} more not shown._`);
  }
}
out.push("");

// Per-file details
out.push("### Per-file results");
out.push("");

const renderFileBlock = (file, byBench) => {
  const lines = [];
  // Runtime issues (missing data, parse errors)
  const issues = [];
  for (const rt of cfg.runtimes) {
    const baseMeta = indexed.get(rt.id)?.base.get(file)?.meta;
    const headMeta = indexed.get(rt.id)?.head.get(file)?.meta;
    if (baseMeta?.parseError) {
      issues.push(`base/${rt.label}: ${baseMeta.parseError}`);
    }
    if (headMeta?.parseError) {
      issues.push(`head/${rt.label}: ${headMeta.parseError}`);
    }
    if (baseMeta?.exitCode != null && baseMeta.exitCode !== 0) {
      issues.push(`base/${rt.label}: non-zero exit (${baseMeta.exitCode})`);
    }
    if (headMeta?.exitCode != null && headMeta.exitCode !== 0) {
      issues.push(`head/${rt.label}: non-zero exit (${headMeta.exitCode})`);
    }
  }

  lines.push(`<details><summary><code>${file}</code></summary>`);
  lines.push("");
  if (issues.length > 0) {
    lines.push("**Issues:**");
    lines.push("");
    for (const iss of issues) {
      lines.push(`- ${iss}`);
    }
    lines.push("");
  }
  // header row
  const header = ["Benchmark"];
  for (const rt of cfg.runtimes) {
    header.push(`${rt.label} base`);
    header.push(`${rt.label} head`);
    header.push("Δ");
  }
  lines.push(`| ${header.join(" | ")} |`);
  lines.push(`|${header.map(() => "---").join("|")}|`);
  for (const [, { alias, args, perRuntime }] of byBench) {
    const argsStr = formatArgs(args);
    const benchLabel = argsStr ? `${alias} \`${argsStr}\`` : alias;
    const cells = [benchLabel];
    for (const rt of cfg.runtimes) {
      const entry = perRuntime.get(rt.id);
      cells.push(formatNs(entry?.base));
      cells.push(formatNs(entry?.head));
      cells.push(
        entry?.delta?.text
          ? `${statusIcon(entry.delta.kind)} ${entry.delta.text}`
          : "-",
      );
    }
    lines.push(`| ${cells.join(" | ")} |`);
  }
  lines.push("");
  lines.push("</details>");
  return lines.join("\n");
};

// We may have to truncate to fit a single PR comment (GitHub: 65,536 chars).
let currentLength = out.join("\n").length;
let truncated = 0;
const renderedBlocks = [];
for (const file of sortedFiles) {
  const block = renderFileBlock(file, rows.get(file));
  if (currentLength + block.length + 200 > COMMENT_BUDGET) {
    truncated++;
    continue;
  }
  renderedBlocks.push(block);
  currentLength += block.length + 2;
}

out.push(...renderedBlocks);
out.push("");
if (truncated > 0) {
  out.push(
    `_${truncated} file section(s) omitted to fit the GitHub comment size limit. Full results are available as workflow artifacts._`,
  );
}

writeFileSync(outPath, out.join("\n"));
console.error(
  `Wrote ${outPath} (${currentLength} chars, flagged=${flagged.length}, files=${sortedFiles.length}, truncated=${truncated})`,
);
