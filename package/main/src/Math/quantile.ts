import { multiplication } from "./multiplication";
import { percentile } from "./percentile";

/**
Calculates the q-th quantile of values in an array using linear interpolation
@param {number[]} values Array of numbers
@param {number} q Quantile (0-1), equivalent to percentile(values, q * 100)
@returns {number} The quantile value, or NaN for an empty array
@example quantile([1, 2, 3, 4, 5], 0.5); // 3
@example quantile([1, 2, 3, 4, 5], 0.25); // 2
@example quantile([1, 2, 3, 4], 0.5); // 2.5
*/
export const quantile = (values: number[], q: number): number =>
  percentile(values, multiplication(q, 100));
