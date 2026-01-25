/**
 * Extract common elements from multiple arrays
 * @param {T[]} array The first array
 * @param {T[][]} arrays Additional arrays to compare
 * @returns {O} Array containing common elements
 * @example getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]
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

  const uniqueResult: unknown[] = [];

  // Use a Set to track seen items for uniqueness if the input array is large,
  // otherwise just check the result array which is faster for small sizes.
  const useSeenSet = array.length > THRESHOLD;
  const seen = useSeenSet ? new Set<unknown>() : undefined;

  for (const item of array) {
    // Check uniqueness
    if (seen) {
      if (seen.has(item)) {
        continue;
      }
    } else if (uniqueResult.includes(item)) {
      continue;
    }

    // Check if item is present in all other arrays/sets
    let isCommon = true;
    for (const collection of otherCollections) {
      if (collection instanceof Set) {
        if (!collection.has(item)) {
          isCommon = false;
          break;
        }
      } else if (!collection.includes(item)) {
        isCommon = false;
        break;
      }
    }

    if (isCommon) {
      if (seen) {
        seen.add(item);
      }
      uniqueResult.push(item);
    }
  }

  return uniqueResult as unknown as O;
};
