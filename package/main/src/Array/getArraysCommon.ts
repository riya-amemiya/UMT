import { orderedUnique } from "./setHelpers/orderedUnique";

/**
Extract common elements from multiple arrays
@param {T[]} array The first array
@param {T[][]} arrays Additional arrays to compare
@returns {O} Array containing common elements
@example getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]
*/
export const getArraysCommon = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
): O => {
  if (arrays.length === 0) {
    return array as unknown as O;
  }

  const THRESHOLD = 120;

  // Convert other arrays to Sets for O(1) lookup if they are large enough
  const otherCollections = arrays.map((array_) =>
    array_.length > THRESHOLD ? new Set(array_) : array_,
  );

  return orderedUnique<unknown>([array]).filter((item) =>
    otherCollections.every((collection) =>
      collection instanceof Set
        ? collection.has(item)
        : collection.includes(item),
    ),
  ) as unknown as O;
};
