/**
 * Removes duplicate values from an array
 * @param array - The array to process
 * @returns A new array with unique values
 */
export const unique = <T>(array: T[]): T[] => {
  return [...new Set(array)];
};
