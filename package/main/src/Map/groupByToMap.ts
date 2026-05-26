/**
 * Groups elements of an array into a Map based on a given iteratee function.
 * Unlike groupBy which returns Record<K, T[]>, this preserves insertion order
 * and allows any key type (including objects and symbols).
 * @param array Array to group
 * @param iteratee Function to determine the group key for each element
 * @returns Map with grouped elements keyed by the iteratee result
 * @example
 * groupByToMap([6.1, 4.2, 6.3], Math.floor); // Map { 6 => [6.1, 6.3], 4 => [4.2] }
 * groupByToMap(["one", "two", "three"], (str) => str.length); // Map { 3 => ['one', 'two'], 5 => ['three'] }
 */
export const groupByToMap = <T, K>(
  array: T[],
  iteratee: (value: T, index: number, array: T[]) => K,
): Map<K, T[]> => {
  const result = new Map<K, T[]>();
  const length = array.length;
  for (let index = 0; index < length; index++) {
    const value = array[index];
    const key = iteratee(value, index, array);
    const bucket = result.get(key);
    if (bucket === undefined) {
      result.set(key, [value]);
    } else {
      bucket.push(value);
    }
  }
  return result;
};
