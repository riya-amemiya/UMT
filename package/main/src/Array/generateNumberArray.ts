/**
 * Generates an array of numbers with the specified length
 * @param length The length of the array
 * @param min The minimum value (default: 0)
 * @param max The maximum value (default: length - 1)
 * @param random Whether to generate random values (default: false)
 * @returns Array of numbers
 * @example generateNumberArray(5); // [0, 1, 2, 3, 4]
 * @example generateNumberArray(5, 10, 14); // [10, 11, 12, 13, 14]
 */
export const generateNumberArray = (
  length: number,
  min = 0,
  max = length - 1,
  random = false,
): number[] => {
  if (length <= 0) {
    return [];
  }
  if (min > max) {
    throw new Error("min should be less than or equal to max");
  }
  if (length === 1) {
    return [min];
  }

  if (random) {
    return Array.from(
      { length },
      () => Math.floor(Math.random() * (max - min + 1)) + min,
    );
  }

  const step = (max - min) / (length - 1);
  return Array.from({ length }, (_, index) => min + index * step);
};
