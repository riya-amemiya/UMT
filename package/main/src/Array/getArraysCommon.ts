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

  const result = array.filter((item: unknown) => {
    // Handle NaN specially since NaN !== NaN
    if (Number.isNaN(item)) {
      return arrays.every((currentArray) =>
        currentArray.some((arrayItem) => Number.isNaN(arrayItem)),
      );
    }
    return arrays.every((currentArray) => currentArray.includes(item));
  });

  const uniqueResult: unknown[] = [];
  for (const item of result) {
    if (Number.isNaN(item)) {
      if (!uniqueResult.some((existing) => Number.isNaN(existing))) {
        uniqueResult.push(item);
      }
    } else if (!uniqueResult.includes(item)) {
      uniqueResult.push(item);
    }
  }

  return uniqueResult as unknown as O;
};
