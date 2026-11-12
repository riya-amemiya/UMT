import { orderedUnique } from "./setHelpers/orderedUnique";

/**
Extract elements that are not common between arrays
@param  {T[]} array The first array
@param  {T[]} arrays Additional arrays to compare
@returns {unknown[]} Array containing elements that appear only once across all arrays
@example getArraysDiff([1, 2, 3], [2, 3, 4]); // [1, 4]
*/
export const getArraysDiff = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
): O => {
  const duplicates = new Set<unknown>();
  const merged = orderedUnique<unknown>(
    [new Set(array), ...arrays],
    undefined,
    (value) => {
      duplicates.add(value);
    },
  );
  const result: unknown[] = [];
  for (const value of merged) {
    if (!duplicates.has(value)) {
      result.push(value === 0 ? 0 : value);
    }
  }
  return result as unknown as O;
};
