/**
 * Returns a random float in the half-open interval `[min, max)`.
 * @param {number} min - Inclusive lower bound
 * @param {number} max - Exclusive upper bound
 * @returns {number} Random float
 * @example
 * randomFloat(0, 1); // 0.000... up to but not including 1
 */
export const randomFloat = (min: number, max: number): number =>
  Math.random() * (max - min) + min;
