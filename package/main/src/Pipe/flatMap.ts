/**
Creates a pipe operator that maps each array element and flattens the result by one level
@param {(value: T, index: number) => U | readonly U[]} function_ - The function that returns a value or an array for each element
@returns {(array: readonly T[]) => U[]} An operator that returns the flattened array
@example
pipe([1, 2], flatMap((x) => [x, x * 10])); // [1, 10, 2, 20]
*/
export const flatMap =
  <T, U>(
    function_: (value: T, index: number) => U | readonly U[],
  ): ((array: readonly T[]) => U[]) =>
  (array: readonly T[]): U[] =>
    array.flatMap((value, index) => function_(value, index));
