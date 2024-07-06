import { readFileSync, writeFileSync } from "node:fs";

// package.jsonを読み込む
const packageJson = JSON.parse(readFileSync("package.json", "utf8"));

// nameをumt-commonに変更
packageJson.name = "umt-common";

// common-moduleにコピー
writeFileSync(
  "common-module/package.json",
  JSON.stringify(packageJson, null, 2),
);

console.log(
  "package.jsonのnameをumt-commonに変更し、common-moduleにコピーしました。",
);
