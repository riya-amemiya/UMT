import { arraysJoin } from './arraysJoin';
import { getArraysCommon } from './getArraysCommon';
import { getArraysDiff } from './getArraysDiff';
import { quickSort } from './quickSort';
import { sum } from './sum';

export { arraysJoin, getArraysCommon, getArraysDiff, quickSort, sum };

export class UMTArrayClass {
    #Local_arraysJoin: typeof arraysJoin;
    #Local_getArraysCommon: typeof getArraysCommon;
    #Local_getArraysDiff: typeof getArraysDiff;
    #Local_quickSort: typeof quickSort;
    #Local_sum: typeof sum;
    constructor() {
        this.#Local_arraysJoin = arraysJoin;
        this.#Local_getArraysCommon = getArraysCommon;
        this.#Local_getArraysDiff = getArraysDiff;
        this.#Local_quickSort = quickSort;
        this.#Local_sum = sum;
    }

    get arraysJoin() {
        return this.#Local_arraysJoin;
    }
    get getArraysCommon() {
        return this.#Local_getArraysCommon;
    }
    get getArraysDiff() {
        return this.#Local_getArraysDiff;
    }
    get quickSort() {
        return this.#Local_quickSort;
    }
    get sum() {
        return this.#Local_sum;
    }
}

export const UMT_Array = new UMTArrayClass();
