import UMT from '../../module';
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
    [UMT.Math.addition(1.1, 2.111), 3.211],
    [UMT.Array.arraysJoin([1, 2], [3, 4]), [1, 2, 3, 4]],
    [UMT.Math.average([1, 2, 3, 4]), 2.5],
    [UMT.Math.calculator('(2*2)'), '4'],
    [UMT.Math.calculator('1.1+2.111'), '3.211'],
    [UMT.Math.calculator('(((2+2)*4)+2)/2'), '9'],
    [UMT.Math.degToRad(90), Math.PI / 2],
    [UMT.Math.division(1.1, 1.11), 0.9],
    [UMT.Math.euclideanAlgorithm(910, 2190, 2121), 1],
    [UMT.Math.factorial(5), 120],
    [UMT.Math.factorize(24), [2, 2, 2, 3]],
    [UMT.Array.getArraysDiff([1, 2, 3, 4], [1, 2, 3]), [4]],
    [
        UMT.Array.getArraysIntersect([1, 2, 3, 4], [1, 2, 3]),
        [1, 2, 3],
    ],
    [UMT.Math.getDecimalLength(1.1), 1],
    [UMT.Math.isDouble(1.1), true],
    [UMT.Math.isDouble('1.1', false), false],
    [UMT.Math.isNumber(1.1), true],
    [UMT.Math.isNumber('1.1', false), false],
    [UMT.Math.isPrimeNumber(2), true],
    [UMT.Math.lcm(2, 3), 6],
    [UMT.Math.max([1, 2, 3, 4]), 4],
    [UMT.Math.min([1, 2, 3, 4]), 1],
    [UMT.Math.multiples(2, 3), [2, 4, 6]],
    [UMT.Math.multiplication(1.1, 2.1), 2.31],
    [UMT.Math.multiplication(0.1, 0.1), 0.01],
    [UMT.Math.nCr(7, 2), 21],
    [UMT.Math.nHr(7, 2), 28],
    [UMT.Math.nPr(7, 2), 42],
    [
        UMT.Math.primeFactorization(24),
        [
            { number: 2, count: 3 },
            { number: 3, count: 1 },
        ],
    ],
    [UMT.Array.quickSort([1, 2, 4, 3]), [1, 2, 3, 4]],
    [UMT.Math.quotient(2.1, 1.2), 1.75],
    [UMT.Math.radToDeg(Math.PI / 2), 90],
    [UMT.Math.reduce(2, 4), { x: 1, y: 2, gcd: 2 }],
    [UMT.Math.repeatedTrial(4, 2, { x: 1, y: 3 }), [8, 27]],
    [UMT.Math.standardDeviation([8, 8, 4, 10, 5, 7]), 2],
    [UMT.Math.subtract(1.1, 2), -0.9],
    [UMT.Math.toBinary(10), '1010'],
    [UMT.Math.valueSwap(2, 1), [1, 2]],
    [
        UMT.Tool.pipeFunction((x: number) => x + 2)(
            (x) => x(3) + 3,
        )(),
        8,
    ],
]);
