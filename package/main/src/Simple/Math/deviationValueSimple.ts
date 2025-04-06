import { average } from "@/Math/average";
import { deviationValue } from "@/Math/deviationValue";
import { standardDeviation } from "@/Math/standardDeviation";

/**
 * Calculate deviation score (T-score)
 * @param  {number} value Input value
 * @param  {number|number[]} averageValue Average value or array of values to calculate average from
 * @param  {number} standardDeviationValue Standard deviation (optional if averageValue is array)
 * @returns {number} Deviation score (50 is average, each standard deviation is worth 10 points)
 * @returns {number} Returns 50 when standard deviation is 0 (all values are the same)
 * @example deviationValueSimple(60, 50, 10); // 60 (one standard deviation above mean)
 * @example deviationValueSimple(60, [40, 50, 60]); // Calculates using array values
 */
export function deviationValueSimple(
  value: number,
  averageValue: number[],
): number;
export function deviationValueSimple(
  value: number,
  averageValue: number,
  standardDeviationValue: number,
): number;
export function deviationValueSimple(
  value: number,
  averageValue: number | number[],
  standardDeviationValue?: number,
): number {
  // Handle array input
  if (Array.isArray(averageValue)) {
    const avg = average(averageValue);
    const sd = standardDeviation(averageValue);
    // When all values are the same, standard deviation is 0
    return sd === 0 ? 50 : deviationValue(value, avg, sd);
  }

  // Handle direct value input with standard deviation
  if (standardDeviationValue === undefined) {
    throw new TypeError(
      "Standard deviation is required when using a single average value",
    );
  }

  return standardDeviationValue === 0
    ? 50
    : deviationValue(value, averageValue, standardDeviationValue);
}
