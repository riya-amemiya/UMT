/**
 * Removes the last element from an array and returns it.
 * @param {T[]} array - The array to remove the last element from.
 * @returns {T | undefined} - The last element of the array, or undefined if the array is empty.
 */
export const pop = <T>(array: T[]): T | undefined => {
  // O(1) performance instead of O(N) from [...array].pop()
  return array.at(-1);
};
