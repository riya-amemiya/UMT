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

  const length1 = string1.length;
  const length2 = string2.length;

  if (length1 === 0) {
    return length2;
  }
  if (length2 === 0) {
    return length1;
  }

  // Ensure string1 is the shorter string to minimize space complexity to O(min(N, M))
  if (length1 > length2) {
    return levenshteinDistance(string2, string1);
  }

  // Create a single row array to store distances
  // We only need the current row and the previous diagonal value
  const row: number[] = Array.from({ length: length1 + 1 });

  // Initialize first row (0 to length1)
  for (let index = 0; index <= length1; index++) {
    row[index] = index;
  }

  // Iterate through each character of string2
  for (let index = 1; index <= length2; index++) {
    let previousDiagonal = row[0]; // Stores the value of matrix[i-1][j-1]
    row[0] = index; // Update first element for the new row (matrix[0][j])

    // Use charCodeAt for integer comparison instead of string indexing,
    // which avoids temporary single-char string allocation per cell.
    // eslint-disable-next-line unicorn/prefer-code-point
    const char2 = string2.charCodeAt(index - 1);

    for (let index = 1; index <= length1; index++) {
      const temporary = row[index]; // Store current value to become prevDiagonal for next iteration
      // eslint-disable-next-line unicorn/prefer-code-point
      const cost = string1.charCodeAt(index - 1) === char2 ? 0 : 1;

      // Manual min avoids variadic Math.min() call overhead per cell.
      // This matches the pattern used in fuzzySearch's inline Levenshtein.
      const deletion = row[index] + 1;
      const insertion = row[index - 1] + 1;
      const substitution = previousDiagonal + cost;

      let value = deletion;
      if (insertion < value) {
        value = insertion;
      }
      if (substitution < value) {
        value = substitution;
      }
      row[index] = value;

      previousDiagonal = temporary;
    }
  }

  return row[length1];
};
