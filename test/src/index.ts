import {
    addition,
    arraysJoin,
    average,
    calculator,
    degToRad,
    division,
    euclideanAlgorithm,
    factorial,
    factorize,
    getArraysDiff,
    getArraysIntersect,
    getDecimalLength,
    isDouble,
    isNumber,
    isPrimeNumber,
    lcm,
    max,
    min,
    multiples,
    multiplication,
    nCr,
    nHr,
    nPr,
    primeFactorization,
    quickSort,
    quotient,
    radToDeg,
    reduce,
    repeatedTrial,
    standardDeviation,
    subtract,
    toBinary,
    valueSwap,
} from '../../module';
let count = 0;
const test = <X extends unknown[][]>(x: X) => {
    for (const i of x) {
        count++;
        if (typeof i[0] != 'object' && !Array.isArray(i[0])) {
            if (!(i[0] === i[1])) {
                throw `Error:${count} out:${i[0]},in:${i[1]}`;
            }
        } else if (Array.isArray(i[0]) && Array.isArray(i[1])) {
            for (let n = 0; n < i[0].length; n++) {
                if (
                    !(
                        JSON.stringify(i[0][n]) ===
                        JSON.stringify(i[1][n])
                    )
                ) {
                    throw `Error:${count} out:${i[0]},in:${i[1]}`;
                }
            }
        } else if (typeof i[0] === 'boolean') {
            if (!i[0] && i[1]) {
                throw `Error:${count} out:${i[0]}:${typeof i[0]},in:${
                    i[1]
                }:${typeof i[1]}`;
            }
        }
    }
};
test([
    [addition(1.1, 2.111), 3.211],
    [arraysJoin([1, 2], [3, 4]), [1, 2, 3, 4]],
    [average([1, 2, 3, 4]), 2.5],
    [calculator('(2*2)'), '4'],
    [calculator('1.1+2.111'), '3.211'],
    [calculator('(((2+2)*4)+2)/2'), '9'],
    [degToRad(90), Math.PI / 2],
    [division(1.1, 1.11), 0.9],
    [euclideanAlgorithm(910, 2190, 2121), 1],
    [factorial(5), 120],
    [factorize(24), [2, 2, 2, 3]],
    [getArraysDiff([1, 2, 3, 4], [1, 2, 3]), [4]],
    [getArraysIntersect([1, 2, 3, 4], [1, 2, 3]), [1, 2, 3]],
    [getDecimalLength(1.1), 1],
    [isDouble(1.1), true],
    [isDouble('1.1', false), false],
    [isNumber(1.1), true],
    [isNumber('1.1', false), false],
    [isPrimeNumber(2), true],
    [lcm(2, 3), 6],
    [max([1, 2, 3, 4]), 4],
    [min([1, 2, 3, 4]), 1],
    [multiples(2, 3), [2, 4, 6]],
    [multiplication(1.1, 2.1), 2.31],
    [multiplication(0.1, 0.1), 0.01],
    [nCr(7, 2), 21],
    [nHr(7, 2), 28],
    [nPr(7, 2), 42],
    [
        primeFactorization(24),
        [
            { number: 2, count: 3 },
            { number: 3, count: 1 },
        ],
    ],
    [quickSort([1, 2, 4, 3]), [1, 2, 3, 4]],
    [quotient(2.1, 1.2), 1.75],
    [radToDeg(Math.PI / 2), 90],
    [reduce(2, 4), { x: 1, y: 2, gcd: 2 }],
    [repeatedTrial(4, 2, { x: 1, y: 3 }), [8, 27]],
    [standardDeviation([8, 8, 4, 10, 5, 7]), 2],
    [subtract(1.1, 2), -0.9],
    [toBinary(10), '1010'],
    [valueSwap(2, 1), [1, 2]],
]);
