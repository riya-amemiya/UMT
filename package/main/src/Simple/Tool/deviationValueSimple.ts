import { average } from "@/Math/average";
import { deviationValue } from "@/Math/deviationValue";
import { standardDeviation } from "@/Math/standardDeviation";

export interface DeviationValueSimple {
  (value: number, averageValue: number, standardDeviationValue: number): number;
  (value: number, averageValue: number[]): number;
}

export const deviationValueSimple = ((
  value: number,
  averageValue: number | number[],
  standardDeviationValue: number,
) => {
  if (Array.isArray(averageValue)) {
    const x = average(averageValue);
    return deviationValue(value, x, standardDeviation(averageValue));
  }
  return deviationValue(value, averageValue, standardDeviationValue);
}) as DeviationValueSimple;
