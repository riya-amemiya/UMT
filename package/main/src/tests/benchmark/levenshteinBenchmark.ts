import { run, bench, group } from "mitata";
import { levenshteinDistance } from "@/String/levenshteinDistance";

group("Levenshtein Distance", () => {
  const str1 = "kitten";
  const str2 = "sitting";
  const longStr1 = "a".repeat(1000);
  const longStr2 = "b".repeat(1000);
  const midStr1 = "Hello World This Is A Test String";
  const midStr2 = "Hello World This Is A Best String";

  // Each input size has a very different cost (short is hundreds of
  // nanoseconds, long is a few milliseconds), so a per-case repetition count is
  // used to bring every measured sample to roughly ~500ms, keeping timer
  // resolution and scheduling jitter negligible.
  const SHORT_ITERATIONS = 2_700_000;
  const MEDIUM_ITERATIONS = 164_000;
  const LONG_ITERATIONS = 180;

  bench("Short strings", () => {
    for (let i = 0; i < SHORT_ITERATIONS; i++) {
      levenshteinDistance(str1, str2);
    }
  });

  bench("Medium strings", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      levenshteinDistance(midStr1, midStr2);
    }
  });

  bench("Long strings (1000 chars)", () => {
    for (let i = 0; i < LONG_ITERATIONS; i++) {
      levenshteinDistance(longStr1, longStr2);
    }
  });
});

await run();
