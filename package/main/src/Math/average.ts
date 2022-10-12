/**
 * 平均値
 * @param  {number[]} numbers
 */
export const average = (numbers: number[]): number => {
    const sum = numbers.reduce((a, b) => a + b, 0);
    const avg = sum / numbers.length;
    return avg;
};
