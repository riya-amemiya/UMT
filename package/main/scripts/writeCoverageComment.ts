import { formatCoverageComment } from "./formatCoverageComment";

const readFlag = (flag: string): string | undefined => {
  const index = Bun.argv.indexOf(flag);
  if (index === -1) {
    return undefined;
  }
  return Bun.argv[index + 1];
};

const readIfExists = async (path: string | undefined): Promise<string> => {
  if (!path) {
    return "";
  }
  const file = Bun.file(path);
  if (!(await file.exists())) {
    return "";
  }
  return file.text();
};

const outPath = readFlag("--out");
if (!outPath) {
  throw new Error("Missing --out");
}

const summaryText = await readIfExists(readFlag("--summary"));
let summary: unknown;
if (summaryText) {
  try {
    summary = JSON.parse(summaryText);
  } catch {
    summary = undefined;
  }
}

const body = formatCoverageComment({
  summary,
  coverageText: await readIfExists(readFlag("--text")),
  lcov: await readIfExists(readFlag("--lcov")),
});

await Bun.write(outPath, body);
