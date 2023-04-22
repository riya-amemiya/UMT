import { addition } from "./addition";
import { average } from "./average";
import { calculator } from "./calculator";
import { calculatorInitialization } from "./calculator/calculatorInitialization";
import { degToRad } from "./degToRad";
import { deviationValue } from "./deviationValue";
import { division } from "./division";
import { factorial } from "./factorial";
import { factorize } from "./factorize";
import { gcd } from "./gcd";
import { getDecimalLength } from "./getDecimalLength";
import { isDouble } from "./isDouble";
import { isNumber } from "./isNumber";
import { isPrimeNumber } from "./isPrimeNumber";
import { lcm } from "./lcm";
import { mathConverter } from "./mathConverter";
import { mathSeparator } from "./mathSeparator";
import { max } from "./max";
import { min } from "./min";
import { multiples } from "./multiples";
import { multiplication } from "./multiplication";
import { nCr } from "./nCr";
import { nHr } from "./nHr";
import { nPr } from "./nPr";
import { primeFactorization } from "./primeFactorization";
import { quotient } from "./quotient";
import { radToDeg } from "./radToDeg";
import { random } from "./random";
import { reduce } from "./reduce";
import { repeatedTrial } from "./repeatedTrial";
import { roundOf } from "./roundOf";
import { solveEquation } from "./solveEquation";
import { standardDeviation } from "./standardDeviation";
import { subtract } from "./subtract";
import { toBinary } from "./toBinary";
import { toCelsius } from "./toCelsius";
import { toKelvin } from "./toKelvin";
import { valueSwap } from "./valueSwap";

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
	solveEquation,
	standardDeviation,
	subtract,
	toBinary,
	toCelsius,
	toKelvin,
	valueSwap,
};
export class UMTMathClass {
	private localAddition: typeof addition;
	private localAverage: typeof average;
	private localCalculator: typeof calculator;
	private localCalculatorInitialization: typeof calculatorInitialization;
	private localDegToRad: typeof degToRad;
	private localDeviationValue: typeof deviationValue;
	private localDivision: typeof division;
	private localFactorial: typeof factorial;
	private localFactorize: typeof factorize;
	private localGcd: typeof gcd;
	private localGetDecimalLength: typeof getDecimalLength;
	private localIsDouble: typeof isDouble;
	private localIsNumber: typeof isNumber;
	private localIsPrimeNumber: typeof isPrimeNumber;
	private localLcm: typeof lcm;
	private localMathConverter: typeof mathConverter;
	private localMathSeparator: typeof mathSeparator;
	private localMax: typeof max;
	private localMin: typeof min;
	private localMultiples: typeof multiples;
	private localMultiplication: typeof multiplication;
	private localNCr: typeof nCr;
	private localNHr: typeof nHr;
	private localNPr: typeof nPr;
	private localPrimeFactorization: typeof primeFactorization;
	private localQuotient: typeof quotient;
	private localRadToDeg: typeof radToDeg;
	private localRandom: typeof random;
	private localReduce: typeof reduce;
	private localRepeatedTrial: typeof repeatedTrial;
	private localRoundOf: typeof roundOf;
	private localSolveEquation: typeof solveEquation;
	private localStandardDeviation: typeof standardDeviation;
	private localSubtract: typeof subtract;
	private localToBinary: typeof toBinary;
	private localToCelsius: typeof toCelsius;
	private localToKelvin: typeof toKelvin;
	private localValueSwap: typeof valueSwap;
	constructor() {
		this.localAddition = addition;
		this.localAverage = average;
		this.localCalculator = calculator;
		this.localCalculatorInitialization = calculatorInitialization;
		this.localDegToRad = degToRad;
		this.localDeviationValue = deviationValue;
		this.localDivision = division;
		this.localFactorial = factorial;
		this.localFactorize = factorize;
		this.localGcd = gcd;
		this.localGetDecimalLength = getDecimalLength;
		this.localIsDouble = isDouble;
		this.localIsNumber = isNumber;
		this.localIsPrimeNumber = isPrimeNumber;
		this.localLcm = lcm;
		this.localMathConverter = mathConverter;
		this.localMathSeparator = mathSeparator;
		this.localMax = max;
		this.localMin = min;
		this.localMultiples = multiples;
		this.localMultiplication = multiplication;
		this.localNCr = nCr;
		this.localNHr = nHr;
		this.localNPr = nPr;
		this.localPrimeFactorization = primeFactorization;
		this.localQuotient = quotient;
		this.localRadToDeg = radToDeg;
		this.localRandom = random;
		this.localReduce = reduce;
		this.localRepeatedTrial = repeatedTrial;
		this.localRoundOf = roundOf;
		this.localSolveEquation = solveEquation;
		this.localStandardDeviation = standardDeviation;
		this.localSubtract = subtract;
		this.localToBinary = toBinary;
		this.localToCelsius = toCelsius;
		this.localToKelvin = toKelvin;
		this.localValueSwap = valueSwap;
	}
	get addition() {
		return this.localAddition;
	}
	get average() {
		return this.localAverage;
	}
	get calculator() {
		return this.localCalculator;
	}
	get calculatorInitialization() {
		return this.localCalculatorInitialization;
	}
	get degToRad() {
		return this.localDegToRad;
	}
	get deviationValue() {
		return this.localDeviationValue;
	}
	get division() {
		return this.localDivision;
	}
	get factorial() {
		return this.localFactorial;
	}
	get factorize() {
		return this.localFactorize;
	}
	get gcd() {
		return this.localGcd;
	}
	get getDecimalLength() {
		return this.localGetDecimalLength;
	}
	get isDouble() {
		return this.localIsDouble;
	}
	get isNumber() {
		return this.localIsNumber;
	}
	get isPrimeNumber() {
		return this.localIsPrimeNumber;
	}
	get lcm() {
		return this.localLcm;
	}
	get mathConverter() {
		return this.localMathConverter;
	}
	get mathSeparator() {
		return this.localMathSeparator;
	}
	get max() {
		return this.localMax;
	}
	get min() {
		return this.localMin;
	}
	get multiples() {
		return this.localMultiples;
	}
	get multiplication() {
		return this.localMultiplication;
	}
	get nCr() {
		return this.localNCr;
	}
	get nHr() {
		return this.localNHr;
	}
	get nPr() {
		return this.localNPr;
	}
	get primeFactorization() {
		return this.localPrimeFactorization;
	}
	get quotient() {
		return this.localQuotient;
	}
	get radToDeg() {
		return this.localRadToDeg;
	}
	get random() {
		return this.localRandom;
	}
	get reduce() {
		return this.localReduce;
	}
	get repeatedTrial() {
		return this.localRepeatedTrial;
	}
	get roundOf() {
		return this.localRoundOf;
	}
	get solveEquation() {
		return this.localSolveEquation;
	}
	get standardDeviation() {
		return this.localStandardDeviation;
	}
	get subtract() {
		return this.localSubtract;
	}
	get toBinary() {
		return this.localToBinary;
	}
	get toCelsius() {
		return this.localToCelsius;
	}
	get toKelvin() {
		return this.localToKelvin;
	}
	get valueSwap() {
		return this.localValueSwap;
	}
}
export const UMT_Math = new UMTMathClass();
