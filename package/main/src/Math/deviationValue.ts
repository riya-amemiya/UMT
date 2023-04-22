/**
 * 偏差値を計算する
 * @param  {number} value
 * @param  {number} averageValue
 * @param  {number} standardDeviationValue
 * @returns number
 */
export const deviationValue = (
    value: number, // current sensor value
    averageValue: number, // average value of all sensor values
    standardDeviationValue: number, // standard deviation of all sensor values
) => {
    return (
        ((value - averageValue) / standardDeviationValue) * 10 + 50
    );
};
