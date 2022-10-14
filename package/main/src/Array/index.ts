import { arraysJoin } from './arraysJoin';
import { getArraysCommon } from './getArraysCommon';
import { getArraysDiff } from './getArraysDiff';
import { quickSort } from './quickSort';
import { sum } from './sum';

export { arraysJoin, getArraysCommon, getArraysDiff, quickSort, sum };

export class UMTArrayClass {
    arraysJoin: typeof arraysJoin;
    getArraysCommon: typeof getArraysCommon;
    getArraysDiff: typeof getArraysDiff;
    quickSort: typeof quickSort;
    sum: typeof sum;
    constructor() {
        this.arraysJoin = arraysJoin;
        this.getArraysCommon = getArraysCommon;
        this.getArraysDiff = getArraysDiff;
        this.quickSort = quickSort;
        this.sum = sum;
    }
}

export const UMT_Array = new UMTArrayClass();
