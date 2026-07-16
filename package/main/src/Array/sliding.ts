/**
 * Returns overlapping windows of a fixed size from an array
 * @param {T[]} array The array to window
 * @param {number} size Window size
 * @param {number} [step=1] Step between window starts
 * @returns {T[][]} Array of windows (incomplete trailing windows are omitted)
 * @example sliding([1, 2, 3, 4, 5], 3); // [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
 * @example sliding([1, 2, 3, 4, 5], 3, 2); // [[1, 2, 3], [3, 4, 5]]
 */
export const sliding = <T>(array: T[], size: number, step = 1): T[][] => {
  const length = array.length;
  const result: T[][] = [];
  for (let index = 0; index + size <= length; index += step) {
    result.push(array.slice(index, index + size));
  }
  return result;
};
