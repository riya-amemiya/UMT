import { average } from '../../Math';
export const deviationValueSimple = (
    value: number,
    averageValue: number | number[],
) => {
    if (Array.isArray(averageValue)) {
        const x = average(averageValue);
        return ((value - x) / x) * 10 + 50;
    } else {
        return ((value - averageValue) / averageValue) * 10 + 50;
    }
};
