import { average } from "./average";

import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";

/**
Calculates the variance of a set of values
@param {number[]} values Array of numeric values
@param {boolean} [sample=false] If true, divides by n - 1 (sample variance) instead of n (population variance)
@returns {number} The variance, 0 for an empty array, or NaN for a sample of a single value
@example variance([1, 2, 3]); // 0.6666666666666666
@example variance([1, 2, 3], true); // 1
@example variance([10, 12, 23, 23, 16, 23, 21, 16]); // 24
*/
export const variance = (values: number[], sample = false): number => {
  if (values.length === 0) {
    return 0;
  }
  if (sample && values.length === 1) {
    return Number.NaN;
  }

  const avg = average(values);
  let sumSquareDiffs = 0;
  for (const value of values) {
    const diff = subtract(value, avg);
    sumSquareDiffs = addition(sumSquareDiffs, multiplication(diff, diff));
  }

  return division(sumSquareDiffs, sample ? values.length - 1 : values.length);
};
