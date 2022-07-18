/* Array */
import arraysJoin from './Array/arraysJoin';
import getArraysDiff from './Array/getArraysDiff';
import getArraysIntersect from './Array/getArraysIntersect';
import quickSort from './Array/quickSort';
import ArrayFunctions from './Array';

/* Math */
import addition from './Math/addition';
import calculator from './Math/calculator';
import degToRad from './Math/degToRad';
import division from './Math/division';
import euclideanAlgorithm from './Math/euclideanAlgorithm';
import factorial from './Math/factorial';
import getDecimalLength from './Math/getDecimalLength';
import isDouble from './Math/isDouble';
import isNumber from './Math/isNumber';
import isPrimeNumber from './Math/isPrimeNumber';
import lcm from './Math/lcm';
import max from './Math/max';
import min from './Math/min';
import multiplication from './Math/multiplication';
import nCr from './Math/nCr';
import nHr from './Math/nHr';
import nPr from './Math/nPr';
import primeFactorization from './Math/primeFactorization';
import quotient from './Math/quotient';
import radToDeg from './Math/radToDeg';
import random from './Math/random';
import reduce from './Math/reduce';
import repeatedTrial from './Math/repeatedTrial';
import subtract from './Math/subtract';
import toBinary from './Math/toBinary';
import valueSwap from './Math/valueSwap';
import average from './Math/average';
import standardDeviation from './Math/standardDeviation';
import factorize from './Math/factorize';
import multiples from './Math/multiples';
import MathFunctions from './Math';

const UMT: {
    Array: typeof ArrayFunctions;
    Math: typeof MathFunctions;
} = {
    Array: ArrayFunctions,
    Math: MathFunctions,
};
export {
    factorial,
    lcm,
    nCr,
    nHr,
    nPr,
    quotient,
    reduce,
    repeatedTrial,
    getArraysIntersect,
    getArraysDiff,
    max,
    min,
    arraysJoin,
    isDouble,
    isNumber,
    valueSwap,
    getDecimalLength,
    subtract,
    multiplication,
    addition,
    division,
    isPrimeNumber,
    toBinary,
    degToRad,
    radToDeg,
    primeFactorization,
    quickSort,
    calculator,
    random,
    euclideanAlgorithm,
    average,
    standardDeviation,
    factorize,
    multiples,
};
export default UMT;
