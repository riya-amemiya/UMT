import { average } from "@/Math/average";
import { deviationValue } from "@/Math/deviationValue";
import { standardDeviation } from "@/Math/standardDeviation";

export interface DeviationValueSimple {
  (value: number, averageValue: number, standardDeviationValue: number): number;
  (value: number, averageValue: number[]): number;
}
/**
 * 偏差値を返す
 * @param  {number} value 値
 * @param  {number|number[]} averageValue 平均値
 * @param  {number} standardDeviationValue 標準偏差
 * @returns number
 * @example deviationValueSimple(60, 50, 10); // 110
 */
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
