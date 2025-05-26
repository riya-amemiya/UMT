/**
 * Default comparison function that returns:
 * - 1 if a > b
 * - -1 if a < b
 * - 0 if a equals b
 * @param {T} a First value to compare
 * @param {T} b Second value to compare
 * @returns {number} Comparison result (-1, 0, or 1)
 * @example
 * compareFunctionDefault(2, 1); // 1
 * compareFunctionDefault(1, 2); // -1
 * compareFunctionDefault(2, 2); // 0
 */
export const compareFunctionDefault = <T>(a: T, b: T): number =>
  a > b ? 1 : a < b ? -1 : 0;
