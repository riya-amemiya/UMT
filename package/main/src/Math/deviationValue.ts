/**
 * 偏差値を計算する
 * @param  {number} value
 * @param  {number} averageValue
 * @returns number
 */
export const deviationValue = (
    value: number,
    averageValue: number,
) => {
    return ((value - averageValue) / averageValue) * 10 + 50;
};
