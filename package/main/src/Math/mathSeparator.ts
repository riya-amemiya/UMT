import { isNumber } from './isNumber';

export const mathSeparator = (number: string | number) => {
    if (isNumber(number)) {
        const [n, x] =
            typeof number === 'string'
                ? [number.length - [1](#1), Number(number)]
                : [String(number).length - [1](#1), number];
        if (n) {
            return [10 ** n, x - [10](#10) ** n];
        }
        return [0, 0];
    }
    return [0, 0];
};
