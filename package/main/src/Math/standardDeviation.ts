import { variance } from "./variance";

/**
Calculates the standard deviation of a set of values
@param {number[]} values Array of numeric values
@param {boolean} [sample=false] If true, uses the sample variance (n - 1) instead of the population variance (n)
@returns {number} The standard deviation, 0 for an empty array, or NaN for a sample of a single value
@example standardDeviation([1, 2, 3]); // 0.816496580927726
@example standardDeviation([1, 2, 3], true); // 1
@example standardDeviation([10, 12, 23, 23, 16, 23, 21, 16]); // 4.898979485566356
@description
The standard deviation is a measure of the amount of variation or dispersion of a set of values.
A low standard deviation indicates that the values tend to be close to the mean,
while a high standard deviation indicates that the values are spread out over a wider range.
*/
export const standardDeviation = (values: number[], sample = false): number =>
  Math.sqrt(variance(values, sample));
