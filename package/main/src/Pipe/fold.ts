/**
Creates a pipe operator that reduces an array to a single value
@param {(accumulator: U, value: T, index: number) => U} reducer - The function that combines the accumulator with each element
@param {U} initialValue - The initial accumulator value
@returns {(array: readonly T[]) => U} An operator that returns the accumulated value
@example
pipe([1, 2, 3], fold((sum, x) => sum + x, 0)); // 6
*/
export const fold =
  <T, U>(
    reducer: (accumulator: U, value: T, index: number) => U,
    initialValue: U,
  ): ((array: readonly T[]) => U) =>
  (array: readonly T[]): U => {
    let accumulator = initialValue;
    const length = array.length;
    for (let index = 0; index < length; index++) {
      accumulator = reducer(accumulator, array[index], index);
    }
    return accumulator;
  };
