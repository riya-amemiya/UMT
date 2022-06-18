import UMT from '../../module';
let count = 0;
const test = <X extends unknown[][]>(x: X) => {
    for (const i of x) {
        count++;
        if (typeof i[0] != 'object' && !Array.isArray(i[0])) {
            if (!(i[0] === i[1])) {
                throw `Error:${count} x:${i[0]},y:${i[1]}`;
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
    [UMT.addition(1.1, 2.111), 3.211],
    [UMT.arraysJoin([1, 2], [3, 4]), [1, 2, 3, 4]],
    [UMT.average([1, 2, 3, 4]), 2.5],
    [UMT.calculator('(2*2)'), '4'],
    [UMT.degToRad(90), Math.PI / 2],
    [UMT.division(1.1, 1.11), 0.9],
    [UMT.euclideanAlgorithm(910, 2190, 2121), 1],
    [UMT.factorial(5), 120],
    [UMT.factorize(24), [2, 2, 2, 3]],
    [UMT.getArraysDiff([1, 2, 3, 4], [1, 2, 3]), [4]],
    [UMT.getArraysIntersect([1, 2, 3, 4], [1, 2, 3]), [1, 2, 3]],
    [UMT.getDecimalLength(1.1), 1],
    [UMT.isDouble(1.1), true],
    [UMT.isDouble('1.1', false), false],
    [UMT.isNumber(1.1), true],
    [UMT.isNumber('1.1', false), false],
    [UMT.isPrimeNumber(2), true],
    [UMT.lcm(2, 3), 6],
    [UMT.max([1, 2, 3, 4]), 4],
    [UMT.min([1, 2, 3, 4]), 1],
    [UMT.multiples(2, 3), [2, 4, 6]],
    [UMT.multiplication(1.1, 2.1), 2.31],
    [UMT.nCr(7, 2), 21],
    [UMT.nHr(7, 2), 28],
    [UMT.nPr(7, 2), 42],
    [
        UMT.primeFactorization(24),
        [
            { number: 2, count: 3 },
            { number: 3, count: 1 },
        ],
    ],
    [UMT.quickSort([1, 2, 4, 3]), [1, 2, 3, 4]],
    [UMT.quotient(2.1, 1.2), 1.75],
    [UMT.radToDeg(Math.PI / 2), 90],
    [UMT.reduce(2, 4), { x: 1, y: 2, gcd: 2 }],
    [UMT.repeatedTrial(4, 2, { x: 1, y: 3 }), [8, 27]],
    [UMT.standardDeviation([8, 8, 4, 10, 5, 7]), 2],
    [UMT.subtract(1.1, 2), -0.9],
    [UMT.toBinary(10), '1010'],
    [UMT.valueSwap(2, 1), [1, 2]],
]);
