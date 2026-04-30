/**
 * Returns a random integer in the closed interval `[min, max]`.
 * @param {number} min - Inclusive lower bound
 * @param {number} max - Inclusive upper bound
 * @returns {number} Random integer
 * @example
 * randomInt(1, 6); // 1, 2, 3, 4, 5, or 6
 */
export const randomInt = (min: number, max: number): number => {
  const lo = Math.ceil(min);
  const hi = Math.floor(max);
  return Math.floor(Math.random() * (hi - lo + 1)) + lo;
};
