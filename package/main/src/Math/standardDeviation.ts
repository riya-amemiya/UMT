import { average } from "./average";

import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
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
  if (values.length === 0) {
    return 0;
  }

  const avg = average(values);

  // Accumulate sum of squared differences in a single pass,
  // avoiding an intermediate array allocation and extra traversal
  let sumSquareDiffs = 0;
  for (const value of values) {
    const diff = subtract(value, avg);
    sumSquareDiffs = addition(sumSquareDiffs, multiplication(diff, diff));
  }

  const avgSquareDiff = division(sumSquareDiffs, values.length);

  return Math.sqrt(avgSquareDiff);
};
