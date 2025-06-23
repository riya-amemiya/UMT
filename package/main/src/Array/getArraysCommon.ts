/**
 * Extract common elements from multiple arrays
 * @param {T[]} array The first array
 * @param {T[][]} arrays Additional arrays to compare
 * @returns {O} Array containing common elements
 * @example getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]
 */
export const getArraysCommon = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
): O => {
  const result = [array, ...arrays].reduce((previous, current) => {
    return previous.filter((item: unknown) => current.includes(item)) as T;
  });
  return result as unknown as O;
};
