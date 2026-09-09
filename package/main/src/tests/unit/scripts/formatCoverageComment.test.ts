import {
  extractCoverageTable,
  filterUncoveredTable,
  formatCoverageComment,
  formatLineRanges,
  parseLcovUncoveredLines,
} from "../../../../scripts/formatCoverageComment";

const JEST_COVERAGE_TEXT = `
PASS src/tests/unit/Date/fromUnix.test.ts
---------------------------------|---------|----------|---------|---------|-------------------
File                             | % Stmts | % Branch | % Funcs | % Lines | Uncovered Line #s 
---------------------------------|---------|----------|---------|---------|-------------------
All files                        |   99.91 |    99.88 |     100 |    99.9 |                   
 Date                            |   95.23 |    96.15 |     100 |   94.73 |                   
  fromUnix.ts                    |   71.42 |       75 |     100 |   66.66 | 12-15              
  toUnix.ts                      |   71.42 |       75 |     100 |   66.66 | 8-10               
 Array                           |     100 |      100 |     100 |     100 |                   
  range.ts                       |     100 |      100 |     100 |     100 |                   
---------------------------------|---------|----------|---------|---------|-------------------
Test Suites: 1 passed, 1 total
`.trim();

const metric = (pct: number) => ({
  total: 10,
  covered: pct === 100 ? 10 : 7,
  skipped: 0,
  pct,
});

const fileSummary = (pct: number) => ({
  statements: metric(pct),
  branches: metric(pct),
  functions: metric(100),
  lines: metric(pct),
});

const summary = {
  total: {
    statements: metric(99.91),
    branches: metric(99.88),
    functions: metric(100),
    lines: metric(99.9),
  },
  "/workspace/package/main/src/Date/fromUnix.ts": fileSummary(71.42),
  "/workspace/package/main/src/Date/toUnix.ts": fileSummary(71.42),
  "/workspace/package/main/src/Array/range.ts": fileSummary(100),
};

const lcov = `
TN:
SF:src/Date/fromUnix.ts
DA:1,1
DA:12,0
DA:13,0
DA:14,0
DA:15,0
BRDA:8,0,0,1
BRDA:8,0,1,0
end_of_record
TN:
SF:src/Date/toUnix.ts
DA:8,0
DA:9,0
DA:10,0
end_of_record
`.trim();

const REAL_JEST_TABLE = `
----------|---------|----------|---------|---------|-------------------
File      | % Stmts | % Branch | % Funcs | % Lines | Uncovered Line #s 
----------|---------|----------|---------|---------|-------------------
All files |   71.42 |       75 |     100 |   66.66 |                   
 toUnix.ts |   71.42 |       75 |     100 |   66.66 | 15-23,24-25       
----------|---------|----------|---------|---------|-------------------
`.trim();

describe("extractCoverageTable", () => {
  it("includes data rows and uncovered line numbers, not just the header", () => {
    const table = extractCoverageTable(JEST_COVERAGE_TEXT);
    expect(table).toContain("fromUnix.ts");
    expect(table).toContain("12-15");
    expect(table).toContain("toUnix.ts");
    expect(table).toContain("8-10");
    expect(table).toContain("All files");
  });

  it("returns empty string when no coverage table is present", () => {
    expect(
      extractCoverageTable("PASS src/tests/unit/Date/fromUnix.test.ts"),
    ).toBe("");
  });

  it("extracts Jest tables whose separator width matches filename padding", () => {
    const table = extractCoverageTable(REAL_JEST_TABLE);
    expect(table).toContain("toUnix.ts");
    expect(table).toContain("15-23,24-25");
  });
});

describe("filterUncoveredTable", () => {
  it("keeps only rows that list uncovered lines", () => {
    const filtered = filterUncoveredTable(
      extractCoverageTable(JEST_COVERAGE_TEXT),
    );
    expect(filtered).toContain("fromUnix.ts");
    expect(filtered).toContain("12-15");
    expect(filtered).not.toContain("range.ts");
    expect(filtered).toContain("File");
  });

  it("returns empty string for a header-only table", () => {
    const headerOnly = [
      "---------------------------------|---------|----------|---------|---------|-------------------",
      "File                             | % Stmts | % Branch | % Funcs | % Lines | Uncovered Line #s ",
      "---------------------------------|---------|----------|---------|---------|-------------------",
    ].join("\n");
    expect(filterUncoveredTable(headerOnly)).toBe("");
  });
});

describe("parseLcovUncoveredLines", () => {
  it("collects zero-hit lines and uncovered branches", () => {
    const parsed = parseLcovUncoveredLines(lcov);
    expect(parsed.get("src/Date/fromUnix.ts")).toEqual([8, 12, 13, 14, 15]);
    expect(parsed.get("src/Date/toUnix.ts")).toEqual([8, 9, 10]);
  });
});

describe("formatLineRanges", () => {
  it("compresses consecutive lines into ranges", () => {
    expect(formatLineRanges([12, 13, 14, 15, 20])).toBe("12-15, 20");
    expect(formatLineRanges([])).toBe("");
    expect(formatLineRanges([3])).toBe("3");
  });
});

describe("formatCoverageComment", () => {
  it("includes uncovered line numbers next to each file below 100%", () => {
    const body = formatCoverageComment({
      summary,
      coverageText: JEST_COVERAGE_TEXT,
      lcov,
    });
    expect(body).toContain("## ❌ カバレッジが100%ではありません");
    expect(body).toContain("| % Stmts | 99.91% | 100% |");
    expect(body).toContain("`src/Date/fromUnix.ts`");
    expect(body).toContain("Uncovered lines: 8, 12-15");
    expect(body).toContain("`src/Date/toUnix.ts`");
    expect(body).toContain("Uncovered lines: 8-10");
    expect(body).not.toContain("`src/Array/range.ts`");
  });

  it("puts uncovered files into the details table instead of a header-only block", () => {
    const body = formatCoverageComment({
      summary,
      coverageText: JEST_COVERAGE_TEXT,
      lcov,
    });
    expect(body).toContain("カバレッジ詳細（未カバーの行を含む）");
    expect(body).toContain("fromUnix.ts");
    expect(body).toContain("12-15");
    const details = body.slice(
      body.indexOf("カバレッジ詳細（未カバーの行を含む）"),
    );
    expect(details).not.toMatch(/```\n[-| ]+\nFile[^\n]+\n[-| ]+\n```/);
  });

  it("still lists uncovered lines from lcov when the text table is missing", () => {
    const body = formatCoverageComment({ summary, lcov });
    expect(body).toContain("Uncovered lines: 8, 12-15");
    expect(body).not.toContain("カバレッジ詳細（未カバーの行を含む）");
  });

  it("writes a fallback comment when the summary cannot be read", () => {
    const body = formatCoverageComment({});
    expect(body).toContain("## ❌ カバレッジが100%ではありません");
    expect(body).toContain("coverage-summary.json");
  });

  it("still attaches the uncovered table when summary metrics are unreadable", () => {
    const body = formatCoverageComment({
      summary: {
        total: {
          statements: { pct: "Unknown" },
          branches: { pct: "Unknown" },
          functions: { pct: "Unknown" },
          lines: { pct: "Unknown" },
        },
      },
      coverageText: REAL_JEST_TABLE,
    });
    expect(body).toContain("coverage-summary.json");
    expect(body).toContain("toUnix.ts");
    expect(body).toContain("15-23,24-25");
  });
});
