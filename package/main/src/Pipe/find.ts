/**
Creates a pipe operator that returns the first array element matching a predicate
@param {(value: T, index: number) => boolean} predicate - The condition the element must satisfy
@returns {(array: readonly T[]) => T | undefined} An operator that returns the first match, or undefined
@example
pipe([1, 2, 3], find((x) => x > 1)); // 2
pipe([1, 2, 3], find((x) => x > 5)); // undefined
*/
export const find =
  <T>(
    predicate: (value: T, index: number) => boolean,
  ): ((array: readonly T[]) => T | undefined) =>
  (array: readonly T[]): T | undefined =>
    array.find((value, index) => predicate(value, index));
