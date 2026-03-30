/**
 * Calculates the Levenshtein distance between two strings
 * Returns the minimum number of single-character edits (insertions, deletions, or substitutions)
 * @param string1 - First string to compare
 * @param string2 - Second string to compare
 * @returns The Levenshtein distance
 */
export const levenshteinDistance = (
  string1: string,
  string2: string,
): number => {
  if (string1 === string2) {
    return 0;
  }

  let short = string1;
  let long = string2;
  let shortLength = short.length;
  let longLength = long.length;

  if (shortLength === 0) {
    return longLength;
  }
  if (longLength === 0) {
    return shortLength;
  }

  // Ensure short is the shorter string to minimize space complexity to O(min(N, M))
  if (shortLength > longLength) {
    short = string2;
    long = string1;
    shortLength = short.length;
    longLength = long.length;
  }

  // Create a single row array to store distances
  // We only need the current row and the previous diagonal value
  const row: number[] = Array.from({ length: shortLength + 1 });

  // Initialize first row (0 to shortLength)
  for (let index = 0; index <= shortLength; index++) {
    row[index] = index;
  }

  // Iterate through each character of the longer string (rows)
  for (let rowIndex = 1; rowIndex <= longLength; rowIndex++) {
    let previousDiagonal = row[0];
    row[0] = rowIndex;

    // eslint-disable-next-line unicorn/prefer-code-point
    const charCode = long.charCodeAt(rowIndex - 1);

    for (let colIndex = 1; colIndex <= shortLength; colIndex++) {
      const temporary = row[colIndex];
      // eslint-disable-next-line unicorn/prefer-code-point
      const cost = short.charCodeAt(colIndex - 1) === charCode ? 0 : 1;

      // Inline min of three values instead of Math.min() call to avoid
      // function-call overhead in this hot loop
      const deletion = row[colIndex] + 1;
      const insertion = row[colIndex - 1] + 1;
      const substitution = previousDiagonal + cost;

      let value = deletion;
      if (insertion < value) {
        value = insertion;
      }
      if (substitution < value) {
        value = substitution;
      }

      row[colIndex] = value;
      previousDiagonal = temporary;
    }
  }

  return row[shortLength];
};
