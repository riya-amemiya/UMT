import { bench, run, summary, do_not_optimize } from "mitata";
import { fuzzySearch } from "@/String/fuzzySearch";
import { levenshteinDistance } from "@/String/levenshteinDistance";
import { quickSort } from "@/Array/quickSort";

// Legacy implementation for comparison
const legacyFuzzySearch = (
  query: string,
  items: string[],
  threshold = 0.6,
): { item: string; score: number }[] => {
  if (query.length === 0) {
    return [];
  }

  const results: { item: string; score: number }[] = [];

  for (const item of items) {
    const distance = levenshteinDistance(
      query.toLowerCase(),
      item.toLowerCase(),
    );
    const maxLength = Math.max(query.length, item.length);
    const score = 1 - distance / maxLength;

    if (score >= threshold) {
      results.push({ item, score });
    }
  }

  return quickSort(results, (a, b) => b.score - a.score);
};

// Setup
const query = "hello";
const itemsSmall = ["hello", "world", "helo", "help", "abcde", "fghij"];
// Generate a mix of similar and dissimilar strings
const itemsLarge = Array.from({ length: 1000 }, (_, index) => {
  const r = Math.random();
  if (r < 0.1) {
    return "hello"; // exact match
  }
  if (r < 0.3) {
    return "helo"; // close match
  }
  if (r < 0.5) {
    return `he${index}`; // partial
  }
  return `randomstring${index}`; // diff length
});

// The small input runs in well under two microseconds and the large input in a
// few hundred microseconds, both noise-dominated. Each group repeats its search
// enough times for one measured sample to take roughly ~500ms, keeping fixed
// overhead negligible. The count is shared across variants within a group.
const SMALL_ITERATIONS = 560_000;
const LARGE_ITERATIONS = 2200;

summary(() => {
  bench("legacyFuzzySearch (small)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(legacyFuzzySearch(query, itemsSmall));
    }
  });

  bench("fuzzySearch (small)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(fuzzySearch(query, itemsSmall));
    }
  });
});

summary(() => {
  bench("legacyFuzzySearch (large)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(legacyFuzzySearch(query, itemsLarge));
    }
  });

  bench("fuzzySearch (large)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(fuzzySearch(query, itemsLarge));
    }
  });
});

(async () => {
  await run();
})();
