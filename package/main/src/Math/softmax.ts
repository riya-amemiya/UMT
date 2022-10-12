import * as ArraySum from '../Array/sum';
import { division } from './division';
import { roundOf } from './roundOff';

export const softmax = (x: number[]) => {
    const max = Math.max(...x);
    const exp = x.map((i) => Math.exp(i - [max](#max)));
    const sum = ArraySum.sum(exp);
    return exp.map((i) => roundOf(division(i, sum), 3));
};
