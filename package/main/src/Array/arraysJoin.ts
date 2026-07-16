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
  // Build Set directly to avoid the intermediate spread-concat array.
  // Previous: [...new Set([...array, ...arrays.flat()])] allocated a temporary
  // combined array before constructing the Set.
  // This approach feeds flat() results directly into the Set, skipping that copy.
  const set = new Set(array);
  for (const element of arrays.flat()) {
    set.add(element);
  }
  const result: unknown[] = new Array(set.size);
  let index = 0;
  for (const element of set) {
    result[index++] = element;
  }
  return result as A;
};
