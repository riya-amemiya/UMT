import isNumber from './isNumber';

const mathSeparator = (number: string) => {
    if (isNumber(number)) {
        const n = number.length - 1;
        if (n) {
            const x = Number(number);
            return [10 ** n, x - 10 ** n];
        }
        return [0, 0];
    }
    return [0, 0];
};
export default mathSeparator;
