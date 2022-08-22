import { average } from '../../Math';

export interface DEVIATIONVALUE {
    (value: number, averageValue: number): number;
    (value: number, averageValue: number[]): number;
}
const deviationValueSimple = ((
    value: number,
    averageValue: number | number[],
) => {
    if (Array.isArray(averageValue)) {
        let x = average(averageValue);
        return ((value - x) / x) * 10 + 50;
    } else {
        return ((value - averageValue) / averageValue) * 10 + 50;
    }
}) as DEVIATIONVALUE;
export default deviationValueSimple;
