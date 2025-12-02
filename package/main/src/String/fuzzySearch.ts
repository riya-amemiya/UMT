import { levenshteinDistance } from "./levenshteinDistance";

import { quickSort } from "@/Array/quickSort";

/**
 * Perform fuzzy string matching on an array of strings
 * @param query - The search query
 * @param items - Array of strings to search in
 * @param threshold - Similarity threshold (0-1) for matching (default: 0.6)
 * @returns Array of matching items with their similarity scores, sorted by best match
 * @example
 * fuzzySearch("hello", ["hello", "world", "helo", "help"]);
 * // [{ item: "hello", score: 1 }, { item: "helo", score: 0.8 }, { item: "help", score: 0.6 }]
 */
export const fuzzySearch = (
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
