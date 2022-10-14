import { addition } from './addition';
import { average } from './average';
import { calculator } from './calculator';
import { degToRad } from './degToRad';
import { deviationValue } from './deviationValue';
import { division } from './division';
import { factorial } from './factorial';
import { factorize } from './factorize';
import { gcd } from './gcd';
import { getDecimalLength } from './getDecimalLength';
import { isDouble } from './isDouble';
import { isNumber } from './isNumber';
import { isPrimeNumber } from './isPrimeNumber';
import { lcm } from './lcm';
import { mathConverter } from './mathConverter';
import { mathSeparator } from './mathSeparator';
import { max } from './max';
import { min } from './min';
import { multiples } from './multiples';
import { multiplication } from './multiplication';
import { nCr } from './nCr';
import { nHr } from './nHr';
import { nPr } from './nPr';
import { primeFactorization } from './primeFactorization';
import { quotient } from './quotient';
import { radToDeg } from './radToDeg';
import { random } from './random';
import { reduce } from './reduce';
import { repeatedTrial } from './repeatedTrial';
import { roundOf } from './roundOff';
import { softmax } from './softmax';
import { standardDeviation } from './standardDeviation';
import { subtract } from './subtract';
import { toBinary } from './toBinary';
import { toCelsius } from './toCelsius';
import { toKelvin } from './toKelvin';
import { valueSwap } from './valueSwap';

export {
    addition,
    average,
    calculator,
    degToRad,
    deviationValue,
    division,
    factorial,
    factorize,
    gcd,
    getDecimalLength,
    isDouble,
    isNumber,
    isPrimeNumber,
    lcm,
    mathConverter,
    mathSeparator,
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
    random,
    reduce,
    repeatedTrial,
    roundOf,
    softmax,
    standardDeviation,
    subtract,
    toBinary,
    toCelsius,
    toKelvin,
    valueSwap,
};
export class UMTMathClass {
    addition: typeof addition;
    average: typeof average;
    calculator: typeof calculator;
    degToRad: typeof degToRad;
    deviationValue: typeof deviationValue;
    division: typeof division;
    factorial: typeof factorial;
    factorize: typeof factorize;
    gcd: typeof gcd;
    getDecimalLength: typeof getDecimalLength;
    isDouble: typeof isDouble;
    isNumber: typeof isNumber;
    isPrimeNumber: typeof isPrimeNumber;
    lcm: typeof lcm;
    mathConverter: typeof mathConverter;
    mathSeparator: typeof mathSeparator;
    max: typeof max;
    min: typeof min;
    multiples: typeof multiples;
    multiplication: typeof multiplication;
    nCr: typeof nCr;
    nHr: typeof nHr;
    nPr: typeof nPr;
    primeFactorization: typeof primeFactorization;
    quotient: typeof quotient;
    radToDeg: typeof radToDeg;
    random: typeof random;
    reduce: typeof reduce;
    repeatedTrial: typeof repeatedTrial;
    roundOf: typeof roundOf;
    softmax: typeof softmax;
    standardDeviation: typeof standardDeviation;
    subtract: typeof subtract;
    toBinary: typeof toBinary;
    toCelsius: typeof toCelsius;
    toKelvin: typeof toKelvin;
    valueSwap: typeof valueSwap;
    constructor() {
        this.addition = addition;
        this.average = average;
        this.calculator = calculator;
        this.degToRad = degToRad;
        this.deviationValue = deviationValue;
        this.division = division;
        this.factorial = factorial;
        this.factorize = factorize;
        this.gcd = gcd;
        this.getDecimalLength = getDecimalLength;
        this.isDouble = isDouble;
        this.isNumber = isNumber;
        this.isPrimeNumber = isPrimeNumber;
        this.lcm = lcm;
        this.mathConverter = mathConverter;
        this.mathSeparator = mathSeparator;
        this.max = max;
        this.min = min;
        this.multiples = multiples;
        this.multiplication = multiplication;
        this.nCr = nCr;
        this.nHr = nHr;
        this.nPr = nPr;
        this.primeFactorization = primeFactorization;
        this.quotient = quotient;
        this.radToDeg = radToDeg;
        this.random = random;
        this.reduce = reduce;
        this.repeatedTrial = repeatedTrial;
        this.roundOf = roundOf;
        this.softmax = softmax;
        this.standardDeviation = standardDeviation;
        this.subtract = subtract;
        this.toBinary = toBinary;
        this.toCelsius = toCelsius;
        this.toKelvin = toKelvin;
        this.valueSwap = valueSwap;
    }
}

export const UMT_Math = new UMTMathClass();
