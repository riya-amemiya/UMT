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
  const queryLength = query.length;
  if (queryLength === 0) {
    return [];
  }

  const results: { item: string; score: number }[] = [];
  const lowerQuery = query.toLowerCase();
  const lowerQueryLength = lowerQuery.length;

  // Pre-allocate row for Levenshtein calculation to avoid repeated allocations.
  // We use lowerQueryLength for the row size, meaning the inner loop will iterate over the query.
  // eslint-disable-next-line unicorn/no-new-array
  const row = new Array(lowerQueryLength + 1);

  nextItem: for (const item of items) {
    const itemLength = item.length;
    const maxLength = Math.max(queryLength, itemLength);

    const maxAllowedDistance = Math.floor(maxLength * (1 - threshold));

    if (Math.abs(queryLength - itemLength) > maxAllowedDistance) {
      continue;
    }

    const lowerItem = item.toLowerCase();
    const lowerItemLength = lowerItem.length;

    // Initialize the row: 0, 1, 2, ..., lowerQueryLength
    for (let index = 0; index <= lowerQueryLength; index++) {
      row[index] = index;
    }

    // Iterate through each character of the lowercased item (outer loop)
    for (let itemIndex = 1; itemIndex <= lowerItemLength; itemIndex++) {
      let previousDiagonal = row[0];
      row[0] = itemIndex;
      const charItem = lowerItem[itemIndex - 1];
      let minRowValue = row[0];

      for (let queryIndex = 1; queryIndex <= lowerQueryLength; queryIndex++) {
        const temporary = row[queryIndex];
        const charQuery = lowerQuery[queryIndex - 1];

        const cost = charQuery === charItem ? 0 : 1;

        row[queryIndex] = Math.min(
          row[queryIndex] + 1, // deletion
          row[queryIndex - 1] + 1, // insertion
          previousDiagonal + cost, // substitution
        );
        previousDiagonal = temporary;

        if (row[queryIndex] < minRowValue) {
          minRowValue = row[queryIndex];
        }
      }

      if (minRowValue > maxAllowedDistance) {
        continue nextItem;
      }
    }

    const distance = row[lowerQueryLength];
    const score = 1 - distance / maxLength;

    if (score >= threshold) {
      results.push({ item, score });
    }
  }

  // Use native sort for better performance and in-place sorting
  // eslint-disable-next-line unicorn/no-array-sort
  return results.sort((a, b) => b.score - a.score);
};
