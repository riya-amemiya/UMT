/**
 * Checks whether a value is null or undefined
 * @param {unknown} value - The value to check
 * @returns {boolean} True if the value is null or undefined
 * @example
 * isNullish(null); // true
 * isNullish(undefined); // true
 * isNullish(0); // false
 * isNullish(""); // false
 */
export const isNullish = (value: unknown): value is null | undefined =>
  value === null || value === undefined;
