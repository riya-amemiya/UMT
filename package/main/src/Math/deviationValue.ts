const deviationValue = (value: number, averageValue: number) => {
    return ((value - averageValue) / averageValue) * 10 + 50;
};
export default deviationValue;
