/**
 * Groups elements of an array based on a given iteratee function
 * @param array Array to group
 * @param iteratee Function to determine the group key for each element
 * @returns Object with grouped elements
 * @example
 * groupBy([6.1, 4.2, 6.3], Math.floor); // { '4': [4.2], '6': [6.1, 6.3] }
 * groupBy(["one", "two", "three"], (str) => str.length); // { '3': ['one', 'two'], '5': ['three'] }
 * groupBy(["apple", "banana", "carrot"], (str) => str[0]); // { 'a': ['apple'], 'b': ['banana'], 'c': ['carrot'] }
 */
export const groupBy = <T, K extends string | number>(
  array: T[],
  iteratee: (value: T, index: number, array: T[]) => K,
): Record<K, T[]> => {
  const result = Object.create(null);
  const length = array.length;
  for (let index = 0; index < length; index++) {
    const value = array[index];
    const key = iteratee(value, index, array);
    // biome-ignore lint/suspicious/noAssignInExpressions: ignore
    (result[key] ?? (result[key] = [])).push(value);
  }
  return result;
};
