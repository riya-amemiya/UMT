/**
Calculates the optimal string alignment (restricted Damerau-Levenshtein) distance
between two strings. Insertions, deletions, substitutions and transpositions of
two adjacent characters each count as one edit, and no substring is edited twice.
Characters are compared as UTF-16 code units, like levenshteinDistance.
Uses three rows sized by the shorter string, so space is O(min(N, M)).
@param string1 - First string to compare
@param string2 - Second string to compare
@returns The optimal string alignment distance
@example
damerauLevenshteinDistance("ca", "ac"); // 1
damerauLevenshteinDistance("kitten", "sitting"); // 3
damerauLevenshteinDistance("ca", "abc"); // 3
*/
export const damerauLevenshteinDistance = (
  string1: string,
  string2: string,
): number => {
  if (string1 === string2) {
    return 0;
  }
  const isFirstShorter = string1.length <= string2.length;
  const short = isFirstShorter ? string1 : string2;
  const long = isFirstShorter ? string2 : string1;
  const shortLength = short.length;
  const longLength = long.length;

  if (shortLength === 0) {
    return longLength;
  }

  const rows: number[][] = [
    Array.from({ length: shortLength + 1 }, (_, index) => index),
    Array.from({ length: shortLength + 1 }, () => 0),
    Array.from({ length: shortLength + 1 }, () => 0),
  ];

  for (let rowIndex = 1; rowIndex <= longLength; rowIndex++) {
    const current = rows[rowIndex % 3];
    const previous = rows[(rowIndex + 2) % 3];
    const beforePrevious = rows[(rowIndex + 1) % 3];
    const longChar = long[rowIndex - 1];
    current[0] = rowIndex;

    for (let colIndex = 1; colIndex <= shortLength; colIndex++) {
      const shortChar = short[colIndex - 1];
      const cost = shortChar === longChar ? 0 : 1;
      const best = Math.min(
        previous[colIndex] + 1,
        current[colIndex - 1] + 1,
        previous[colIndex - 1] + cost,
      );
      current[colIndex] =
        rowIndex > 1 &&
        colIndex > 1 &&
        longChar === short[colIndex - 2] &&
        long[rowIndex - 2] === shortChar &&
        beforePrevious[colIndex - 2] + 1 < best
          ? beforePrevious[colIndex - 2] + 1
          : best;
    }
  }

  return rows[longLength % 3][shortLength];
};
