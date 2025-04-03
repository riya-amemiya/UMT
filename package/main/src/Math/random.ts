/**
 * Generates a random integer between min and max (inclusive)
 * @param  {number} max Maximum value (inclusive)
 * @param  {number} min Minimum value (inclusive, default: 0)
 * @returns {number} Random integer between min and max
 * @example random(10); // returns number between 0 and 10
 * @example random(10, 5); // returns number between 5 and 10
 */
export const random = (max: number, min = 0): number =>
  Math.floor(Math.random() * (max - min + 1)) + min;
