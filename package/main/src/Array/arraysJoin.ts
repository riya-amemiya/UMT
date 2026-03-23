/**
 * Join arrays without duplicates
 * @param  {unknown[]} array - First array to join
 * @param  {unknown[]} arrays - Additional arrays to join
 * @returns {unknown[]} Array with unique elements from all input arrays
 * @example arraysJoin([1, 2, 3], [2, 3, 4]); // [1, 2, 3, 4]
 */
export const arraysJoin = <A extends unknown[]>(
  array: unknown[],
  ...arrays: unknown[]
) => {
  // Build Set directly to avoid intermediate array allocations.
  // Previous: [...new Set([...array, ...arrays.flat()])] created 2 temporary
  // arrays (the spread-concat and flat result) before constructing the Set.
  // This approach populates the Set in-place, eliminating O(n) extra copies.
  const set = new Set(array);
  for (const item of arrays) {
    if (Array.isArray(item)) {
      for (const element of item) {
        set.add(element);
      }
    } else {
      set.add(item);
    }
  }
  return [...set] as A;
};
