"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.euclideanAlgorithm = exports.random = exports.calculator = exports.quickSort = exports.primeFactorization = exports.radToDeg = exports.degToRad = exports.toBinary = exports.isPrimeNumber = exports.division = exports.addition = exports.multiplication = exports.subtract = exports.getDecimalLength = exports.valueSwap = exports.isNumber = exports.isDouble = exports.arraysJoin = exports.min = exports.max = exports.getArraysDiff = exports.getArraysIntersect = exports.repeatedTrial = exports.reduce = exports.quotient = exports.nPr = exports.nCrs = exports.nCr = exports.lcm = exports.fact = void 0;
const addition_1 = __importDefault(require("./addition"));
exports.addition = addition_1.default;
const arraysJoin_1 = __importDefault(require("./arraysJoin"));
exports.arraysJoin = arraysJoin_1.default;
const calculator_1 = __importDefault(require("./calculator"));
exports.calculator = calculator_1.default;
const degToRad_1 = __importDefault(require("./degToRad"));
exports.degToRad = degToRad_1.default;
const division_1 = __importDefault(require("./division"));
exports.division = division_1.default;
const euclideanAlgorithm_1 = __importDefault(require("./euclideanAlgorithm"));
exports.euclideanAlgorithm = euclideanAlgorithm_1.default;
const fact_1 = __importDefault(require("./fact"));
exports.fact = fact_1.default;
const getArraysDiff_1 = __importDefault(require("./getArraysDiff"));
exports.getArraysDiff = getArraysDiff_1.default;
const getArraysIntersect_1 = __importDefault(require("./getArraysIntersect"));
exports.getArraysIntersect = getArraysIntersect_1.default;
const getDecimalLength_1 = __importDefault(require("./getDecimalLength"));
exports.getDecimalLength = getDecimalLength_1.default;
const isDouble_1 = __importDefault(require("./isDouble"));
exports.isDouble = isDouble_1.default;
const isNumber_1 = __importDefault(require("./isNumber"));
exports.isNumber = isNumber_1.default;
const isPrimeNumber_1 = __importDefault(require("./isPrimeNumber"));
exports.isPrimeNumber = isPrimeNumber_1.default;
const lcm_1 = __importDefault(require("./lcm"));
exports.lcm = lcm_1.default;
const max_1 = __importDefault(require("./max"));
exports.max = max_1.default;
const min_1 = __importDefault(require("./min"));
exports.min = min_1.default;
const multiplication_1 = __importDefault(require("./multiplication"));
exports.multiplication = multiplication_1.default;
const nCr_1 = __importDefault(require("./nCr"));
exports.nCr = nCr_1.default;
const nCrs_1 = __importDefault(require("./nCrs"));
exports.nCrs = nCrs_1.default;
const nPr_1 = __importDefault(require("./nPr"));
exports.nPr = nPr_1.default;
const primeFactorization_1 = __importDefault(require("./primeFactorization"));
exports.primeFactorization = primeFactorization_1.default;
const quickSort_1 = __importDefault(require("./quickSort"));
exports.quickSort = quickSort_1.default;
const quotient_1 = __importDefault(require("./quotient"));
exports.quotient = quotient_1.default;
const radToDeg_1 = __importDefault(require("./radToDeg"));
exports.radToDeg = radToDeg_1.default;
const random_1 = __importDefault(require("./random"));
exports.random = random_1.default;
const reduce_1 = __importDefault(require("./reduce"));
exports.reduce = reduce_1.default;
const repeatedTrial_1 = __importDefault(require("./repeatedTrial"));
exports.repeatedTrial = repeatedTrial_1.default;
const subtract_1 = __importDefault(require("./subtract"));
exports.subtract = subtract_1.default;
const toBinary_1 = __importDefault(require("./toBinary"));
exports.toBinary = toBinary_1.default;
const valueSwap_1 = __importDefault(require("./valueSwap"));
exports.valueSwap = valueSwap_1.default;
const UMT = {
    fact: fact_1.default,
    lcm: lcm_1.default,
    nCr: nCr_1.default,
    nCrs: nCrs_1.default,
    nPr: nPr_1.default,
    quotient: quotient_1.default,
    reduce: reduce_1.default,
    repeatedTrial: repeatedTrial_1.default,
    getArraysIntersect: getArraysIntersect_1.default,
    getArraysDiff: getArraysDiff_1.default,
    max: max_1.default,
    min: min_1.default,
    arraysJoin: arraysJoin_1.default,
    isDouble: isDouble_1.default,
    isNumber: isNumber_1.default,
    valueSwap: valueSwap_1.default,
    getDecimalLength: getDecimalLength_1.default,
    subtract: subtract_1.default,
    multiplication: multiplication_1.default,
    addition: addition_1.default,
    division: division_1.default,
    isPrimeNumber: isPrimeNumber_1.default,
    toBinary: toBinary_1.default,
    degToRad: degToRad_1.default,
    radToDeg: radToDeg_1.default,
    primeFactorization: primeFactorization_1.default,
    quickSort: quickSort_1.default,
    calculator: calculator_1.default,
    random: random_1.default,
    euclideanAlgorithm: euclideanAlgorithm_1.default,
};
exports.default = UMT;
