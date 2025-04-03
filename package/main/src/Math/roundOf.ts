/**
 * Rounds a number to specified decimal places
 * @param {number} value - Number to round
 * @param {number} precision - Number of decimal places (default: 0)
 * @returns {number} Rounded number
 * @example roundOf(1.234, 2); // 1.23
 * @example roundOf(1.235, 2); // 1.24
 * @example roundOf(-1.234, 2); // -1.23
 */
export const roundOf = (value: number, precision = 0) => {
  return Math.round(value * 10 ** precision) / 10 ** precision;
};
