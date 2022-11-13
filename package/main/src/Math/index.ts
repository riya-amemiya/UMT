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
    #Localaddition: typeof addition;
    #Localaverage: typeof average;
    #Localcalculator: typeof calculator;
    #LocalcalculatorInitialization: typeof calculatorInitialization;
    #LocaldegToRad: typeof degToRad;
    #LocaldeviationValue: typeof deviationValue;
    #Localdivision: typeof division;
    #Localfactorial: typeof factorial;
    #Localfactorize: typeof factorize;
    #Localgcd: typeof gcd;
    #LocalgetDecimalLength: typeof getDecimalLength;
    #LocalisDouble: typeof isDouble;
    #LocalisNumber: typeof isNumber;
    #LocalisPrimeNumber: typeof isPrimeNumber;
    #Locallcm: typeof lcm;
    #LocalmathConverter: typeof mathConverter;
    #LocalmathSeparator: typeof mathSeparator;
    #Localmax: typeof max;
    #Localmin: typeof min;
    #Localmultiples: typeof multiples;
    #Localmultiplication: typeof multiplication;
    #LocalnCr: typeof nCr;
    #LocalnHr: typeof nHr;
    #LocalnPr: typeof nPr;
    #LocalprimeFactorization: typeof primeFactorization;
    #Localquotient: typeof quotient;
    #LocalradToDeg: typeof radToDeg;
    #Localrandom: typeof random;
    #Localreduce: typeof reduce;
    #LocalrepeatedTrial: typeof repeatedTrial;
    #LocalroundOf: typeof roundOf;
    #Localsoftmax: typeof softmax;
    #LocalstandardDeviation: typeof standardDeviation;
    #Localsubtract: typeof subtract;
    #LocaltoBinary: typeof toBinary;
    #LocaltoCelsius: typeof toCelsius;
    #LocaltoKelvin: typeof toKelvin;
    #LocalvalueSwap: typeof valueSwap;
    constructor() {
        this.#Localaddition = addition;
        this.#Localaverage = average;
        this.#Localcalculator = calculator;
        this.#LocalcalculatorInitialization =
            calculatorInitialization;
        this.#LocaldegToRad = degToRad;
        this.#LocaldeviationValue = deviationValue;
        this.#Localdivision = division;
        this.#Localfactorial = factorial;
        this.#Localfactorize = factorize;
        this.#Localgcd = gcd;
        this.#LocalgetDecimalLength = getDecimalLength;
        this.#LocalisDouble = isDouble;
        this.#LocalisNumber = isNumber;
        this.#LocalisPrimeNumber = isPrimeNumber;
        this.#Locallcm = lcm;
        this.#LocalmathConverter = mathConverter;
        this.#LocalmathSeparator = mathSeparator;
        this.#Localmax = max;
        this.#Localmin = min;
        this.#Localmultiples = multiples;
        this.#Localmultiplication = multiplication;
        this.#LocalnCr = nCr;
        this.#LocalnHr = nHr;
        this.#LocalnPr = nPr;
        this.#LocalprimeFactorization = primeFactorization;
        this.#Localquotient = quotient;
        this.#LocalradToDeg = radToDeg;
        this.#Localrandom = random;
        this.#Localreduce = reduce;
        this.#LocalrepeatedTrial = repeatedTrial;
        this.#LocalroundOf = roundOf;
        this.#Localsoftmax = softmax;
        this.#LocalstandardDeviation = standardDeviation;
        this.#Localsubtract = subtract;
        this.#LocaltoBinary = toBinary;
        this.#LocaltoCelsius = toCelsius;
        this.#LocaltoKelvin = toKelvin;
        this.#LocalvalueSwap = valueSwap;
    }
    get addition() {
        return this.#Localaddition;
    }
    get average() {
        return this.#Localaverage;
    }
    get calculator() {
        return this.#Localcalculator;
    }
    get calculatorInitialization() {
        return this.#LocalcalculatorInitialization;
    }
    get degToRad() {
        return this.#LocaldegToRad;
    }
    get deviationValue() {
        return this.#LocaldeviationValue;
    }
    get division() {
        return this.#Localdivision;
    }
    get factorial() {
        return this.#Localfactorial;
    }
    get factorize() {
        return this.#Localfactorize;
    }
    get gcd() {
        return this.#Localgcd;
    }
    get getDecimalLength() {
        return this.#LocalgetDecimalLength;
    }
    get isDouble() {
        return this.#LocalisDouble;
    }
    get isNumber() {
        return this.#LocalisNumber;
    }
    get isPrimeNumber() {
        return this.#LocalisPrimeNumber;
    }
    get lcm() {
        return this.#Locallcm;
    }
    get mathConverter() {
        return this.#LocalmathConverter;
    }
    get mathSeparator() {
        return this.#LocalmathSeparator;
    }
    get max() {
        return this.#Localmax;
    }
    get min() {
        return this.#Localmin;
    }
    get multiples() {
        return this.#Localmultiples;
    }
    get multiplication() {
        return this.#Localmultiplication;
    }
    get nCr() {
        return this.#LocalnCr;
    }
    get nHr() {
        return this.#LocalnHr;
    }
    get nPr() {
        return this.#LocalnPr;
    }
    get primeFactorization() {
        return this.#LocalprimeFactorization;
    }
    get quotient() {
        return this.#Localquotient;
    }
    get radToDeg() {
        return this.#LocalradToDeg;
    }
    get random() {
        return this.#Localrandom;
    }
    get reduce() {
        return this.#Localreduce;
    }
    get repeatedTrial() {
        return this.#LocalrepeatedTrial;
    }
    get roundOf() {
        return this.#LocalroundOf;
    }
    get softmax() {
        return this.#Localsoftmax;
    }
    get standardDeviation() {
        return this.#LocalstandardDeviation;
    }
    get subtract() {
        return this.#Localsubtract;
    }
    get toBinary() {
        return this.#LocaltoBinary;
    }
    get toCelsius() {
        return this.#LocaltoCelsius;
    }
    get toKelvin() {
        return this.#LocaltoKelvin;
    }
    get valueSwap() {
        return this.#LocalvalueSwap;
    }
}
export const UMT_Math = new UMTMathClass();
