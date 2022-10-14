/**
 * 偏差値を計算する
 * @param  {number} value
 * @param  {number} averageValue
 * @param  {number} standardDeviationValue
 * @returns number
 */
export const deviationValue = (
    value: number,
    averageValue: number,
    standardDeviationValue: number,
) => {
    return (
        ((value - averageValue) / standardDeviationValue) * 10 + 50
    );
};
