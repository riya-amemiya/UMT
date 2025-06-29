/**
 * Calculates the Levenshtein distance between two strings
 * Returns the minimum number of single-character edits (insertions, deletions, or substitutions)
 * @param str1 - First string to compare
 * @param str2 - Second string to compare
 * @returns The Levenshtein distance
 */
export const levenshteinDistance = (
  string1: string,
  string2: string,
): number => {
  const length1 = string1.length;
  const length2 = string2.length;

  // Return the length of the other string if one is empty
  if (length1 === 0) {
    return length2;
  }
  if (length2 === 0) {
    return length1;
  }

  // Create a 2D array for dynamic programming
  const matrix: number[][] = Array.from({ length: length1 + 1 }, () =>
    Array.from({ length: length2 + 1 }, () => 0),
  );

  // Initialize first column and row
  for (let index = 0; index <= length1; index++) {
    matrix[index][0] = index;
  }
  for (let index = 0; index <= length2; index++) {
    matrix[0][index] = index;
  }

  // Calculate distances
  for (let index = 1; index <= length1; index++) {
    for (let index_ = 1; index_ <= length2; index_++) {
      const cost = string1[index - 1] === string2[index_ - 1] ? 0 : 1;
      matrix[index][index_] = Math.min(
        matrix[index - 1][index_] + 1, // deletion
        matrix[index][index_ - 1] + 1, // insertion
        matrix[index - 1][index_ - 1] + cost, // substitution
      );
    }
  }

  return matrix[length1][length2];
};
