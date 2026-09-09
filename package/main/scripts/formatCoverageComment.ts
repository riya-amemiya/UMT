type CoverageMetric = {
  pct: number;
};

type FileCoverageSummary = {
  statements: CoverageMetric;
  branches: CoverageMetric;
  functions: CoverageMetric;
  lines: CoverageMetric;
};

export type CoverageCommentInput = {
  summary?: unknown;
  coverageText?: string;
  lcov?: string;
};

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null;

const isMetric = (value: unknown): value is CoverageMetric =>
  isRecord(value) && typeof value.pct === "number";

const isFileCoverage = (value: unknown): value is FileCoverageSummary =>
  isRecord(value) &&
  isMetric(value.statements) &&
  isMetric(value.branches) &&
  isMetric(value.functions) &&
  isMetric(value.lines);

const isSeparatorLine = (line: string): boolean =>
  /^-+(?:\|[-|\s]+)+$/.test(line.trimEnd());

const separatorIndexes = (lines: string[]): number[] => {
  const indexes: number[] = [];
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    if (line !== undefined && isSeparatorLine(line)) {
      indexes.push(index);
    }
  }
  return indexes;
};

const toSrcPath = (file: string): string => file.replace(/.*\/src\//, "src/");

const uncoveredFor = (
  file: string,
  byPath: Map<string, number[]>,
): number[] => {
  const direct = byPath.get(file);
  if (direct) {
    return direct;
  }
  const srcPath = toSrcPath(file);
  for (const [key, lines] of byPath) {
    if (toSrcPath(key) === srcPath) {
      return lines;
    }
  }
  return [];
};

const isBelowFullCoverage = (data: FileCoverageSummary): boolean =>
  data.statements.pct < 100 ||
  data.branches.pct < 100 ||
  data.functions.pct < 100 ||
  data.lines.pct < 100;

const issueList = (data: FileCoverageSummary): string[] => {
  const issues: string[] = [];
  if (data.statements.pct < 100) {
    issues.push(`Stmts: ${data.statements.pct}%`);
  }
  if (data.branches.pct < 100) {
    issues.push(`Branch: ${data.branches.pct}%`);
  }
  if (data.functions.pct < 100) {
    issues.push(`Funcs: ${data.functions.pct}%`);
  }
  if (data.lines.pct < 100) {
    issues.push(`Lines: ${data.lines.pct}%`);
  }
  return issues;
};

const detailsTable = (table: string): string[] => [
  "",
  "<details>",
  "<summary>カバレッジ詳細（未カバーの行を含む）</summary>",
  "",
  "```",
  table,
  "```",
  "</details>",
];

const fallbackComment = (table: string): string => {
  const parts = [
    "## ❌ カバレッジが100%ではありません",
    "",
    "coverage-summary.json を読み取れませんでした。CIログのカバレッジ出力を確認してください。",
  ];
  if (table) {
    parts.push(...detailsTable(table));
  }
  return `${parts.join("\n")}\n`;
};

export const extractCoverageTable = (coverageText: string): string => {
  const lines = coverageText.split(/\r?\n/);
  const separators = separatorIndexes(lines);
  if (separators.length < 2) {
    return "";
  }
  const start = separators[0];
  const end = separators[separators.length - 1];
  if (start === undefined || end === undefined) {
    return "";
  }
  return lines.slice(start, end + 1).join("\n");
};

export const filterUncoveredTable = (table: string): string => {
  if (!table) {
    return "";
  }
  const lines = table.split(/\r?\n/);
  const separators = separatorIndexes(lines);
  if (separators.length < 3) {
    return "";
  }
  const headerStart = separators[0];
  const headerEnd = separators[1];
  const footer = separators[separators.length - 1];
  if (
    headerStart === undefined ||
    headerEnd === undefined ||
    footer === undefined
  ) {
    return "";
  }
  const kept = lines.slice(headerStart, headerEnd + 1);
  for (let index = headerEnd + 1; index < footer; index++) {
    const line = lines[index];
    if (line === undefined) {
      continue;
    }
    const cells = line.split("|");
    const last = cells[cells.length - 1];
    if (last !== undefined && last.trim() !== "") {
      kept.push(line);
    }
  }
  if (kept.length === headerEnd - headerStart + 1) {
    return "";
  }
  const footerLine = lines[footer];
  if (footerLine !== undefined) {
    kept.push(footerLine);
  }
  return kept.join("\n");
};

export const parseLcovUncoveredLines = (
  lcov: string,
): Map<string, number[]> => {
  const result = new Map<string, number[]>();
  const records = lcov.split("end_of_record");
  for (const record of records) {
    const sfMatch = /(?:^|\n)SF:(.+)/.exec(record);
    const file = sfMatch?.[1]?.trim();
    if (!file) {
      continue;
    }
    const uncovered = new Set<number>();
    for (const match of record.matchAll(/^DA:(\d+),(\d+)/gm)) {
      const line = Number(match[1]);
      const hits = Number(match[2]);
      if (hits === 0 && Number.isFinite(line)) {
        uncovered.add(line);
      }
    }
    for (const match of record.matchAll(/^BRDA:(\d+),\d+,\d+,(\d+|-)$/gm)) {
      const line = Number(match[1]);
      const taken = match[2];
      if ((taken === "0" || taken === "-") && Number.isFinite(line)) {
        uncovered.add(line);
      }
    }
    if (uncovered.size > 0) {
      result.set(
        file,
        [...uncovered].sort((left, right) => left - right),
      );
    }
  }
  return result;
};

export const formatLineRanges = (lines: number[]): string => {
  const sorted = [...new Set(lines)].sort((left, right) => left - right);
  const first = sorted[0];
  if (first === undefined) {
    return "";
  }
  const parts: string[] = [];
  let rangeStart = first;
  let previous = first;
  for (let index = 1; index < sorted.length; index++) {
    const current = sorted[index];
    if (current === undefined) {
      continue;
    }
    if (current === previous + 1) {
      previous = current;
      continue;
    }
    parts.push(
      rangeStart === previous ? `${rangeStart}` : `${rangeStart}-${previous}`,
    );
    rangeStart = current;
    previous = current;
  }
  parts.push(
    rangeStart === previous ? `${rangeStart}` : `${rangeStart}-${previous}`,
  );
  return parts.join(", ");
};

export const formatCoverageComment = (input: CoverageCommentInput): string => {
  const summary = isRecord(input.summary) ? input.summary : undefined;
  const total = summary?.total;
  const table = filterUncoveredTable(
    extractCoverageTable(input.coverageText ?? ""),
  );
  if (!isFileCoverage(total)) {
    return fallbackComment(table);
  }

  const parts = [
    "## ❌ カバレッジが100%ではありません",
    "",
    "| Metric | Coverage | Required |",
    "|--------|----------|----------|",
    `| % Stmts | ${total.statements.pct}% | 100% |`,
    `| % Branch | ${total.branches.pct}% | 100% |`,
    `| % Funcs | ${total.functions.pct}% | 100% |`,
    `| % Lines | ${total.lines.pct}% | 100% |`,
  ];

  const lcovMap = parseLcovUncoveredLines(input.lcov ?? "");
  const uncoveredFiles: string[] = [];
  if (summary) {
    for (const [file, data] of Object.entries(summary)) {
      if (file === "total" || !isFileCoverage(data)) {
        continue;
      }
      if (!isBelowFullCoverage(data)) {
        continue;
      }
      const line = `- \`${toSrcPath(file)}\`: ${issueList(data).join(", ")}`;
      const ranges = formatLineRanges(uncoveredFor(file, lcovMap));
      uncoveredFiles.push(
        ranges ? `${line}\n  - Uncovered lines: ${ranges}` : line,
      );
    }
  }

  if (uncoveredFiles.length > 0) {
    parts.push("", "### カバレッジが100%未満のファイル", "", ...uncoveredFiles);
  }

  if (table) {
    parts.push(...detailsTable(table));
  }

  return `${parts.join("\n")}\n`;
};
