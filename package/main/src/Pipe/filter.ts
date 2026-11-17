/**
Creates a pipe operator that keeps the array elements matching a predicate
@param {(value: T, index: number) => boolean} predicate - The condition each element must satisfy
@returns {(array: readonly T[]) => T[]} An operator that returns the matching elements
@example
pipe([1, 2, 3, 4], filter((x) => x % 2 === 0)); // [2, 4]
*/
export const filter =
  <T>(
    predicate: (value: T, index: number) => boolean,
  ): ((array: readonly T[]) => T[]) =>
  (array: readonly T[]): T[] =>
    array.filter((value, index) => predicate(value, index));
