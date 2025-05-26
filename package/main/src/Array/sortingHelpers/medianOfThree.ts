import type { CompareFunction } from "$/array/compareFunction";

/**
 * Find median of three elements
 */
export const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: CompareFunction<T>,
): number => {
  const ab = compareFunction(array[a], array[b]);
  if (ab < 0) {
    const bc = compareFunction(array[b], array[c]);
    if (bc < 0) {
      return b;
    }
    const ac = compareFunction(array[a], array[c]);
    return ac < 0 ? c : a;
  }
  const bc = compareFunction(array[b], array[c]);
  if (bc > 0) {
    return b;
  }
  const ac = compareFunction(array[a], array[c]);
  return ac > 0 ? c : a;
};
