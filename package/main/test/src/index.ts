import {
    arraysJoin,
    getArraysCommon,
    getArraysDiff,
    quickSort,
} from '../../module/Array';
import sum from '../../module/Array/sum';
import {
    addition,
    average,
    calculator,
    degToRad,
    division,
    factorial,
    factorize,
    gcd,
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
    quotient,
    radToDeg,
    reduce,
    repeatedTrial,
    roundOf,
    standardDeviation,
    subtract,
    toBinary,
    valueSwap,
} from '../../module/Math';
import mathConverter from '../../module/Math/mathConverter';
import softmax from '../../module/Math/softmax';
import { pipeFunction } from '../../module/Tool';
const dummyFunction = <T>(x: T) => {
    return x;
};
const test = <
    F extends (...args: any) => any,
    X extends [[F, ...Parameters<F>], ReturnType<F>][],
>(
    x: X,
) => {
    const test = x.map((i) => {
        const args = [];
        for (let j = 1; j < i[0].length; j++) {
            args.push(i[0][j]);
        }
        return [i[0][0](...args), i[1], i[0][0].name];
    });
    for (const i of test) {
        if (typeof i[0] === 'function') {
            if (!(i[0]() === i[1])) {
                throw `Error:${i[2]} type:${typeof i[0]} out:${
                    i[0]
                },in:${i[1]}`;
            }
        } else if (typeof i[0] != 'object' && !Array.isArray(i[0])) {
            if (!(i[0] === i[1])) {
                throw `Error:${i[2]} out:${i[0]},in:${i[1]}`;
            }
        } else if (Array.isArray(i[0]) && Array.isArray(i[1])) {
            for (let n = 0; n < i[0].length; n++) {
                if (
                    !(
                        JSON.stringify(i[0][n]) ===
                        JSON.stringify(i[1][n])
                    )
                ) {
                    throw `Error:${i[2]} type:${typeof i[0]} out:${
                        i[0]
                    },in:${i[1]}`;
                }
            }
        } else if (typeof i[0] === 'boolean') {
            if (!i[0] && i[1]) {
                throw `Error:${i[2]} type:${typeof i[0]} out:${
                    i[0]
                }:${typeof i[0]},in:${i[1]}:${typeof i[1]}`;
            }
        }
    }
};
test([
    [[addition, 1.1, 2.111], 3.211],
    [
        [arraysJoin, [1, 2], [3, 4]],
        [1, 2, 3, 4],
    ],
    [[average, [1, 2, 3, 4]], 2.5],
    [[calculator, '(2*2)'], '4'],
    [[calculator, '1.1+2.111'], '3.211'],
    [[calculator, '(((2+2)*4)+2)/2'], '9'],
    [[calculator, '(1+1)^3'], '8'],
    [[calculator, '(((2+2)*4)+2)/$2', { $: 100 }], '0.09'],
    [[degToRad, 90], Math.PI / 2],
    [[division, 1.1, 1.1], 1],
    [[roundOf, division(1.1, 2.111), 3], 0.521],
    [[factorial, 5], 120],
    [
        [factorize, 12],
        [2, 2, 3],
    ],
    [[gcd, 910, 2190, 2121], 1],
    [[factorial, 5], 120],
    [
        [factorize, 24],
        [2, 2, 2, 3],
    ],
    [
        [getArraysDiff, [1, 2, 3], [2, 3, 4], [3, 4, 5]],
        [1, 5],
    ],
    [[getArraysCommon, [1, 2, 3], [2, 3, 4], [3, 4, 5]], [3]],
    [[getDecimalLength, 1.1], 1],
    [[isDouble, 1.1], true],
    [[isDouble, '1.1', false], false],
    [[isNumber, 1.1], true],
    [[isNumber, '1.1', false], false],
    [[isNumber, '1.1'], true],
    [[isPrimeNumber, 2], true],
    [[lcm, 2, 3], 6],
    [[max, 1, 2, 3, 4], 4],
    [[min, 1, 2, 3, 4], 1],
    [
        [multiples, 2, 3],
        [2, 4, 6],
    ],
    [[multiplication, 1.1, 2.1], 2.31],
    [[multiplication, 0.1, 0.1], 0.01],
    [[multiplication, 8.91, 5.3], 47.223],
    [[nCr, 7, 2], 21],
    [[nHr, 7, 2], 28],
    [[nPr, 7, 2], 42],
    [
        [primeFactorization, 24],
        [
            { number: 2, count: 3 },
            { number: 3, count: 1 },
        ],
    ],
    [
        [quickSort, [1, 2, 4, 3]],
        [1, 2, 3, 4],
    ],
    [[quotient, 2.1, 1.2], 1.75],
    [[radToDeg, Math.PI / 2], 90],
    [[reduce, 2, 4], { x: 1, y: 2, gcd: 2 }],
    [
        [repeatedTrial, 4, 2, { x: 1, y: 3 }],
        [8, 27],
    ],
    [[standardDeviation, [8, 8, 4, 10, 5, 7]], 2],
    [[subtract, 1.1, 2], -0.9],
    [[toBinary, 10], '1010'],
    [
        [valueSwap, 2, 1],
        [1, 2],
    ],
    [
        [
            dummyFunction,
            pipeFunction((x: number) => x + 2)(
                (x) => (y: number) => x(3) + y,
            )((x: (x: number) => any) => x(4))(),
        ],
        9,
    ],
    [
        [
            dummyFunction,
            pipeFunction(() => 3 + 2)((x: () => any) => x() + 3),
        ],
        8,
    ],
    [[mathConverter, '1250*1250'], '1500*1000+400*100+200*100+50*50'],
    [[sum, softmax([1, 2, 4])], 1],
]);
console.log('====================================');
console.log(
    average(
        Array(9)
            .fill(3)
            .concat(Array(8).fill(4).concat(Array(7).fill(5))),
    ),
);
console.log('====================================');
