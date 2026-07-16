/**
 * Counts elements of an array grouped by an iteratee key
 * @param {T[]} array Array to count
 * @param {(value: T, index: number, array: T[]) => K} iteratee
 *   Function to determine the group key for each element
 * @returns {Record<K, number>} Object mapping keys to counts
 * @example countBy([6.1, 4.2, 6.3], Math.floor); // { '4': 1, '6': 2 }
 * @example countBy(["one", "two", "three"], (s) => s.length); // { '3': 2, '5': 1 }
 */
export const countBy = <T, K extends string | number>(
  array: T[],
  iteratee: (value: T, index: number, array: T[]) => K,
): Record<K, number> => {
  const result = Object.create(null) as Record<K, number>;
  const length = array.length;
  for (let index = 0; index < length; index++) {
    const key = iteratee(array[index], index, array);
    result[key] = (result[key] ?? 0) + 1;
  }
  return result;
};
