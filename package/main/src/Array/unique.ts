/**
 * Removes duplicate values from an array
 * @param array - The array to process
 * @returns A new array with unique values
 */
export const unique = <T>(array: T[]): T[] => {
  const seen = new Set<T>();
  const result: T[] = [];
  const length = array.length;
  for (let index = 0; index < length; index++) {
    const value = array[index];
    if (!seen.has(value)) {
      seen.add(value);
      result.push(value);
    }
  }
  return result;
};
