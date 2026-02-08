/**
 * Checks whether a value is null or undefined
 * @param {unknown} value - The value to check
 * @returns {boolean} True if the value is null or undefined
 * @example
 * isNotNullish(null); // false
 * isNotNullish(undefined); // false
 * isNotNullish(0); // true
 * isNotNullish(""); // true
 */
export const isNotNullish = <T>(
  value: T,
): value is Exclude<T, null | undefined> =>
  value !== null && value !== undefined;
