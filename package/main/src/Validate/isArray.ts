/**
 * Determines if the value is an array
 * @param array - Value to check
 * @returns boolean - True if the value is an array, false otherwise
 * @example isArray([1, 2, 3]); // true
 * isArray({}); // false
 */
export const isArray = <T>(array: unknown): array is T[] => {
  return Array.isArray(array);
};
