/**
 * Returns a random boolean. The probability of `true` defaults to 0.5.
 * @param {number} [probability=0.5] - Probability of returning true (0..1)
 * @returns {boolean} Random boolean
 * @example
 * randomBoolean(); // true or false equally
 * randomBoolean(0.9); // true ~90% of the time
 */
export const randomBoolean = (probability = 0.5): boolean =>
  Math.random() < probability;
