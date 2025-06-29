import { average } from "./average";

import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";

/**
 * Calculates the standard deviation of a set of values
 * @param {number[]} values Array of numeric values
 * @returns {number} The standard deviation
 * @example standardDeviation([1, 2, 3]); // 0.816496580927726
 * @example standardDeviation([10, 12, 23, 23, 16, 23, 21, 16]); // 4.898979485566356
 * @description
 * The standard deviation is a measure of the amount of variation or dispersion of a set of values.
 * A low standard deviation indicates that the values tend to be close to the mean,
 * while a high standard deviation indicates that the values are spread out over a wider range.
 */
export const standardDeviation = (values: number[]): number => {
  const avg = average(values);

  // Calculate the squared differences from the mean
  const squareDiffs = values.map((value) => {
    const diff = subtract(value, avg);
    return multiplication(diff, diff);
  });

  // Calculate the mean of the squared differences
  const avgSquareDiff = average(squareDiffs);

  // Return the square root of the mean squared differences
  return Math.sqrt(avgSquareDiff);
};
