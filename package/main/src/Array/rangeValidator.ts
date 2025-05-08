export interface ValidatedSortRange {
  startIndex: number;
  endIndex: number;
  shouldSort: boolean;
}

/**
 * Validates and adjusts the start and end indices for an operation on an array.
 * Ensures indices are within the bounds of the array.
 *
 * @template T The type of elements in the array.
 * @param {T[]} array The array.
 * @param {number} startIndex The desired starting index.
 * @param {number} endIndex The desired ending index.
 * @returns {ValidatedSortRange} An object containing the validated start index, end index,
 *                               and a boolean indicating if the operation should proceed on the range.
 */
export const validateRange = <T>(
  // Renamed from validateSortRange for more general use
  array: T[],
  startIndex: number,
  endIndex: number,
): ValidatedSortRange => {
  const length = array.length;
  // Handle empty arrays early
  if (length === 0) {
    return { startIndex: 0, endIndex: -1, shouldSort: false };
  }

  const validatedStartIndex = Math.max(0, Math.min(startIndex, length - 1));
  const validatedEndIndex = Math.max(
    validatedStartIndex,
    Math.min(endIndex, length - 1),
  );

  return {
    startIndex: validatedStartIndex,
    endIndex: validatedEndIndex,
    shouldSort: validatedEndIndex >= validatedStartIndex,
  };
};
