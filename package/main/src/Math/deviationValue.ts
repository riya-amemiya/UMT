/**
 * 偏差値を計算する
 * @param  {number} value - 現在のセンサー値
 * @param  {number} averageValue - センサー値の平均値
 * @param  {number} standardDeviationValue - センサー値の標準偏差
 * @returns {number} 偏差値
 * @example deviationValue(10, 5, 2); // 75
 */
export const deviationValue = (
  value: number,
  averageValue: number,
  standardDeviationValue: number,
): number => {
  return ((value - averageValue) / standardDeviationValue) * 10 + 50;
};
