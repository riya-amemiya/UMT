/**
 * Removes duplicate values from an array based on a selector function
 * @param array - The array to process
 * @param selector - Function that returns the value to compare for uniqueness
 * @returns A new array with unique values based on the selector
 */
export const uniqBy = <T extends unknown[], K>(
  array: T,
  selector: (item: T[number]) => K,
): T => {
  const seen = new Set<K>();
  const result: T = [] as unknown as T;

  for (const item of array) {
    const key = selector(item as T);
    if (!seen.has(key)) {
      seen.add(key);
      result.push(item);
    }
  }

  return result;
};
