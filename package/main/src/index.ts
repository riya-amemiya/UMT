//Array
import arraysJoin from './Array/arraysJoin';
import getArraysDiff from './Array/getArraysDiff';
import getArraysCommon from './Array/getArraysCommon';
import quickSort from './Array/quickSort';

//Date
import now from './Date/now';

//Math
import factorial from './Math/factorial';
import lcm from './Math/lcm';
import nCr from './Math/nCr';
import nHr from './Math/nHr';
import nPr from './Math/nPr';
import quotient from './Math/quotient';
import reduce from './Math/reduce';
import repeatedTrial from './Math/repeatedTrial';
import max from './Math/max';
import min from './Math/min';
import isDouble from './Math/isDouble';
import isNumber from './Math/isNumber';
import valueSwap from './Math/valueSwap';
import getDecimalLength from './Math/getDecimalLength';
import subtract from './Math/subtract';
import multiplication from './Math/multiplication';
import addition from './Math/addition';
import division from './Math/division';
import isPrimeNumber from './Math/isPrimeNumber';
import toBinary from './Math/toBinary';
import degToRad from './Math/degToRad';
import radToDeg from './Math/radToDeg';
import primeFactorization from './Math/primeFactorization';
import calculator from './Math/calculator';
import random from './Math/random';
import gcd from './Math/gcd';
import average from './Math/average';
import standardDeviation from './Math/standardDeviation';
import factorize from './Math/factorize';
import multiples from './Math/multiples';
import toKelvin from './Math/toKelvin';
import toCelsius from './Math/toCelsius';

//Tool
import birthday from './Tool/birthday';
import dayOfWeek from './Tool/dayOfWeek';
import pipeFunction from './Tool/pipeFunction';

//Simple Date
import nowSimple from './Simple/Date/now';

//Simple Math
import dayOfWeekSimple from './Simple/Math/dayOfWeek';
import deviationValueSimple from './Simple/Math/deviationValue';

//Simple Tool
import birthdaySimple from './Simple/Tool/birthday';

const UMT = {
    Array: { arraysJoin, getArraysDiff, getArraysCommon, quickSort },
    Date: { now },
    Math: {
        factorial,
        lcm,
        nCr,
        nHr,
        nPr,
        quotient,
        reduce,
        repeatedTrial,
        max,
        min,
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
        calculator,
        random,
        gcd,
        average,
        standardDeviation,
        factorize,
        multiples,
        toKelvin,
        toCelsius,
    },

    Tool: { birthday, dayOfWeek, pipeFunction },

    Simple: {
        Date: { nowSimple },
        Math: { dayOfWeekSimple, deviationValueSimple },
        Tool: { birthdaySimple },
    },
};

export default UMT;
