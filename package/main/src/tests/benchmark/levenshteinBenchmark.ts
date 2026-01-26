import { run, bench, group } from "mitata";
import { levenshteinDistance } from "@/String/levenshteinDistance";

group("Levenshtein Distance", () => {
  const str1 = "kitten";
  const str2 = "sitting";
  const longStr1 = "a".repeat(1000);
  const longStr2 = "b".repeat(1000);
  const midStr1 = "Hello World This Is A Test String";
  const midStr2 = "Hello World This Is A Best String";

  bench("Short strings", () => {
    levenshteinDistance(str1, str2);
  });

  bench("Medium strings", () => {
    levenshteinDistance(midStr1, midStr2);
  });

  bench("Long strings (1000 chars)", () => {
    levenshteinDistance(longStr1, longStr2);
  });
});

await run();
