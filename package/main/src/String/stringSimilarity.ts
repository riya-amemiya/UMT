import { levenshteinDistance } from "./levenshteinDistance";

/**
 * Calculates the similarity between two strings as a percentage
 * Uses Levenshtein distance normalized by the length of the longer string
 * @param str1 - First string to compare
 * @param str2 - Second string to compare
 * @returns Similarity score between 0 (completely different) and 1 (identical)
 */
export const stringSimilarity = (string1: string, string2: string): number => {
  // Identical strings have 100% similarity
  if (string1 === string2) {
    return 1;
  }

  // Empty strings handling
  if (string1.length === 0 || string2.length === 0) {
    return 0;
  }

  const distance = levenshteinDistance(string1, string2);
  const maxLength = Math.max(string1.length, string2.length);

  // Calculate similarity as 1 - (distance / maxLength)
  return 1 - distance / maxLength;
};
