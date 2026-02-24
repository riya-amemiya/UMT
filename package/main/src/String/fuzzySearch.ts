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
  const lowerQuery = query.toLowerCase();
  const queryLength = query.length;

  // Reusable row array to avoid allocations
  // We allocate enough space for typical use cases or grow it if needed.
  // eslint-disable-next-line unicorn/no-new-array
  let rowBuffer = new Array(Math.max(queryLength, 50) + 1);

  for (const item of items) {
    const itemLength = item.length;
    const maxLength = itemLength > queryLength ? itemLength : queryLength;

    const maxAllowedDistance = Math.floor(maxLength * (1 - threshold));

    if (Math.abs(queryLength - itemLength) > maxAllowedDistance) {
      continue;
    }

    const lowerItem = item.toLowerCase();

    // Inline Levenshtein with early exit
    // We want to transform lowerQuery -> lowerItem (or vice versa)
    // To minimize space, we use the shorter string for the columns (row array).
    let string1 = lowerQuery;
    let string2 = lowerItem;
    let length1 = queryLength;
    let length2 = itemLength;

    if (length1 > length2) {
      string1 = lowerItem;
      string2 = lowerQuery;
      length1 = itemLength;
      length2 = queryLength;
    }

    // Ensure buffer is large enough
    if (rowBuffer.length <= length1) {
      // eslint-disable-next-line unicorn/no-new-array
      rowBuffer = new Array(length1 + 1);
    }

    // Initialize first row: 0, 1, 2, ..., length1
    for (let j = 0; j <= length1; j++) {
      rowBuffer[j] = j;
    }

    let minDistanceInRow = 0;

    // Iterate through each character of string2 (rows)
    for (let i = 1; i <= length2; i++) {
      let previousDiagonal = rowBuffer[0];
      rowBuffer[0] = i;

      const char2 = string2.charCodeAt(i - 1);
      // Reset min distance for this row
      minDistanceInRow = rowBuffer[0];

      for (let j = 1; j <= length1; j++) {
        const temporary = rowBuffer[j];
        const cost = string1.charCodeAt(j - 1) === char2 ? 0 : 1;

        // min(deletion, insertion, substitution)
        // deletion: row[j] (from previous row)
        // insertion: row[j-1] (current row)
        // substitution: previousDiagonal + cost
        const deletion = rowBuffer[j] + 1;
        const insertion = rowBuffer[j - 1] + 1;
        const substitution = previousDiagonal + cost;

        let value = deletion;
        if (insertion < value) {
          value = insertion;
        }
        if (substitution < value) {
          value = substitution;
        }

        rowBuffer[j] = value;
        previousDiagonal = temporary;

        if (value < minDistanceInRow) {
          minDistanceInRow = value;
        }
      }

      // Early exit if the minimum distance in this row exceeds the max allowed
      if (minDistanceInRow > maxAllowedDistance) {
        break;
      }
    }

    if (minDistanceInRow <= maxAllowedDistance) {
      const trueDistance = rowBuffer[length1];
      if (trueDistance <= maxAllowedDistance) {
        const score = 1 - trueDistance / maxLength;
        // Check threshold one more time (redundant if logic is perfect but safe)
        if (score >= threshold) {
          results.push({ item, score });
        }
      }
    }
  }

  return quickSort(results, (a, b) => b.score - a.score);
};
