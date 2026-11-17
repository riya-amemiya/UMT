/**
Creates a pipe operator that returns a section of an array
@param {number} [start] - The index to start from, 0 when omitted
@param {number} [end] - The index to end before, the array length when omitted
@returns {<T>(array: readonly T[]) => T[]} An operator that returns the selected elements
@example
pipe([1, 2, 3, 4], slice(1, 3)); // [2, 3]
pipe([1, 2, 3, 4], slice(-2)); // [3, 4]
*/
export const slice =
  (start?: number, end?: number): (<T>(array: readonly T[]) => T[]) =>
  <T>(array: readonly T[]): T[] =>
    array.slice(start, end);
