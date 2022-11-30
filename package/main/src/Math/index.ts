import { addition } from './addition';
import { average } from './average';
import { calculator } from './calculator';
import { calculatorInitialization } from './calculator/calculatorInitialization';
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
import { roundOf } from './roundOf';
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
    calculatorInitialization,
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
    #Local_addition: typeof addition;
    #Local_average: typeof average;
    #Local_calculator: typeof calculator;
    #Local_calculatorInitialization: typeof calculatorInitialization;
    #Local_degToRad: typeof degToRad;
    #Local_deviationValue: typeof deviationValue;
    #Local_division: typeof division;
    #Local_factorial: typeof factorial;
    #Local_factorize: typeof factorize;
    #Local_gcd: typeof gcd;
    #Local_getDecimalLength: typeof getDecimalLength;
    #Local_isDouble: typeof isDouble;
    #Local_isNumber: typeof isNumber;
    #Local_isPrimeNumber: typeof isPrimeNumber;
    #Local_lcm: typeof lcm;
    #Local_mathConverter: typeof mathConverter;
    #Local_mathSeparator: typeof mathSeparator;
    #Local_max: typeof max;
    #Local_min: typeof min;
    #Local_multiples: typeof multiples;
    #Local_multiplication: typeof multiplication;
    #Local_nCr: typeof nCr;
    #Local_nHr: typeof nHr;
    #Local_nPr: typeof nPr;
    #Local_primeFactorization: typeof primeFactorization;
    #Local_quotient: typeof quotient;
    #Local_radToDeg: typeof radToDeg;
    #Local_random: typeof random;
    #Local_reduce: typeof reduce;
    #Local_repeatedTrial: typeof repeatedTrial;
    #Local_roundOf: typeof roundOf;
    #Local_softmax: typeof softmax;
    #Local_standardDeviation: typeof standardDeviation;
    #Local_subtract: typeof subtract;
    #Local_toBinary: typeof toBinary;
    #Local_toCelsius: typeof toCelsius;
    #Local_toKelvin: typeof toKelvin;
    #Local_valueSwap: typeof valueSwap;
    constructor() {
        this.#Local_addition = addition;
        this.#Local_average = average;
        this.#Local_calculator = calculator;
        this.#Local_calculatorInitialization =
            calculatorInitialization;
        this.#Local_degToRad = degToRad;
        this.#Local_deviationValue = deviationValue;
        this.#Local_division = division;
        this.#Local_factorial = factorial;
        this.#Local_factorize = factorize;
        this.#Local_gcd = gcd;
        this.#Local_getDecimalLength = getDecimalLength;
        this.#Local_isDouble = isDouble;
        this.#Local_isNumber = isNumber;
        this.#Local_isPrimeNumber = isPrimeNumber;
        this.#Local_lcm = lcm;
        this.#Local_mathConverter = mathConverter;
        this.#Local_mathSeparator = mathSeparator;
        this.#Local_max = max;
        this.#Local_min = min;
        this.#Local_multiples = multiples;
        this.#Local_multiplication = multiplication;
        this.#Local_nCr = nCr;
        this.#Local_nHr = nHr;
        this.#Local_nPr = nPr;
        this.#Local_primeFactorization = primeFactorization;
        this.#Local_quotient = quotient;
        this.#Local_radToDeg = radToDeg;
        this.#Local_random = random;
        this.#Local_reduce = reduce;
        this.#Local_repeatedTrial = repeatedTrial;
        this.#Local_roundOf = roundOf;
        this.#Local_softmax = softmax;
        this.#Local_standardDeviation = standardDeviation;
        this.#Local_subtract = subtract;
        this.#Local_toBinary = toBinary;
        this.#Local_toCelsius = toCelsius;
        this.#Local_toKelvin = toKelvin;
        this.#Local_valueSwap = valueSwap;
    }
    get addition() {
        return this.#Local_addition;
    }
    get average() {
        return this.#Local_average;
    }
    get calculator() {
        return this.#Local_calculator;
    }
    get calculatorInitialization() {
        return this.#Local_calculatorInitialization;
    }
    get degToRad() {
        return this.#Local_degToRad;
    }
    get deviationValue() {
        return this.#Local_deviationValue;
    }
    get division() {
        return this.#Local_division;
    }
    get factorial() {
        return this.#Local_factorial;
    }
    get factorize() {
        return this.#Local_factorize;
    }
    get gcd() {
        return this.#Local_gcd;
    }
    get getDecimalLength() {
        return this.#Local_getDecimalLength;
    }
    get isDouble() {
        return this.#Local_isDouble;
    }
    get isNumber() {
        return this.#Local_isNumber;
    }
    get isPrimeNumber() {
        return this.#Local_isPrimeNumber;
    }
    get lcm() {
        return this.#Local_lcm;
    }
    get mathConverter() {
        return this.#Local_mathConverter;
    }
    get mathSeparator() {
        return this.#Local_mathSeparator;
    }
    get max() {
        return this.#Local_max;
    }
    get min() {
        return this.#Local_min;
    }
    get multiples() {
        return this.#Local_multiples;
    }
    get multiplication() {
        return this.#Local_multiplication;
    }
    get nCr() {
        return this.#Local_nCr;
    }
    get nHr() {
        return this.#Local_nHr;
    }
    get nPr() {
        return this.#Local_nPr;
    }
    get primeFactorization() {
        return this.#Local_primeFactorization;
    }
    get quotient() {
        return this.#Local_quotient;
    }
    get radToDeg() {
        return this.#Local_radToDeg;
    }
    get random() {
        return this.#Local_random;
    }
    get reduce() {
        return this.#Local_reduce;
    }
    get repeatedTrial() {
        return this.#Local_repeatedTrial;
    }
    get roundOf() {
        return this.#Local_roundOf;
    }
    get softmax() {
        return this.#Local_softmax;
    }
    get standardDeviation() {
        return this.#Local_standardDeviation;
    }
    get subtract() {
        return this.#Local_subtract;
    }
    get toBinary() {
        return this.#Local_toBinary;
    }
    get toCelsius() {
        return this.#Local_toCelsius;
    }
    get toKelvin() {
        return this.#Local_toKelvin;
    }
    get valueSwap() {
        return this.#Local_valueSwap;
    }
}
export const UMT_Math = new UMTMathClass();
