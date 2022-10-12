import * as ArraySum from '../Array/sum';
import division from './division';
import roundOf from './roundOff';

const softmax = (x: number[]) => {
    const max = Math.max(...x);
    const exp = x.map((i) => Math.exp(i - max));
    const sum = ArraySum.default(exp);
    return exp.map((i) => roundOf(division(i, sum), 3));
};

export default softmax;