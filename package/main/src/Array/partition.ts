/**
 * Splits an array into two arrays based on a predicate
 * @param {T[]} array The array to partition
 * @param {(value: T, index: number, array: T[]) => boolean} predicate
 *   Function returning true for elements that go into the first array
 * @returns {[T[], T[]]} A tuple of [pass, fail] arrays
 * @example partition([1, 2, 3, 4], (n) => n % 2 === 0); // [[2, 4], [1, 3]]
 */
export const partition = <T>(
  array: T[],
  predicate: (value: T, index: number, array: T[]) => boolean,
): [T[], T[]] => {
  const pass: T[] = [];
  const fail: T[] = [];
  const length = array.length;
  for (let index = 0; index < length; index++) {
    const value = array[index];
    if (predicate(value, index, array)) {
      pass.push(value);
    } else {
      fail.push(value);
    }
  }
  return [pass, fail];
};
