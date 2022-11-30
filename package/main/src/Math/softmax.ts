import * as ArraySum from '../Array/sum';
import { division } from './division';
import { roundOf } from './roundOf';
/**
 * @param  {number[]} x
 * @return {number[]}
 */
export const softmax = (x: number[]): number[] => {
    const max = Math.max(...x);
    const exp = x.map((i) => Math.exp(i - max));
    const sum = ArraySum.sum(exp);
    return exp.map((i) => roundOf(division(i, sum), 3));
};
