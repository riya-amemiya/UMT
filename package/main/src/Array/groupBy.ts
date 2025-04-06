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
  return array.reduce(
    (accumulator, value, index, array) => {
      const key = iteratee(value, index, array);
      if (!accumulator[key]) {
        accumulator[key] = [];
      }
      accumulator[key].push(value);
      return accumulator;
    },
    {} as Record<K, T[]>,
  );
};
