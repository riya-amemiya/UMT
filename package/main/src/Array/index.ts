import { arraysJoin } from './arraysJoin';
import { getArraysCommon } from './getArraysCommon';
import { getArraysDiff } from './getArraysDiff';
import { quickSort } from './quickSort';
import { sum } from './sum';

export { arraysJoin, getArraysCommon, getArraysDiff, quickSort, sum };

export class UMTArrayClass {
    #LocalarraysJoin: typeof arraysJoin;
    #LocalgetArraysCommon: typeof getArraysCommon;
    #LocalgetArraysDiff: typeof getArraysDiff;
    #LocalquickSort: typeof quickSort;
    #Localsum: typeof sum;
    constructor() {
        this.#LocalarraysJoin = arraysJoin;
        this.#LocalgetArraysCommon = getArraysCommon;
        this.#LocalgetArraysDiff = getArraysDiff;
        this.#LocalquickSort = quickSort;
        this.#Localsum = sum;
    }

    get arraysJoin() {
        return this.#LocalarraysJoin;
    }
    get getArraysCommon() {
        return this.#LocalgetArraysCommon;
    }
    get getArraysDiff() {
        return this.#LocalgetArraysDiff;
    }
    get quickSort() {
        return this.#LocalquickSort;
    }
    get sum() {
        return this.#Localsum;
    }
}

export const UMT_Array = new UMTArrayClass();
