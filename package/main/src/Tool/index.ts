import { birthday } from './birthday';
import { dayOfWeek } from './dayOfWeek';
import { isBrowser } from './isBrowser';
import { isNode } from './isNode';
import { isNodeWebkit } from './isNodeWebkit';
import { pipeFunction } from './pipeFunction';

export { birthday, dayOfWeek, pipeFunction };

export class UMTToolClass {
    #Local_birthday: typeof birthday;
    #Local_dayOfWeek: typeof dayOfWeek;
    #Local_pipeFunction: typeof pipeFunction;
    #Local_isBrowser: boolean;
    #Local_isNode: boolean;
    #Local_isNodeWebkit: boolean;
    constructor() {
        this.#Local_birthday = birthday;
        this.#Local_dayOfWeek = dayOfWeek;
        this.#Local_pipeFunction = pipeFunction;
        this.#Local_isBrowser = isBrowser;
        this.#Local_isNode = isNode;
        this.#Local_isNodeWebkit = isNodeWebkit;
    }
    get birthday() {
        return this.#Local_birthday;
    }
    get dayOfWeek() {
        return this.#Local_dayOfWeek;
    }
    get pipeFunction() {
        return this.#Local_pipeFunction;
    }
    get isBrowser() {
        return this.#Local_isBrowser;
    }
    get isNode() {
        return this.#Local_isNode;
    }
    get isNodeWebkit() {
        return this.#Local_isNodeWebkit;
    }
}

export const UMT_Tool = new UMTToolClass();
