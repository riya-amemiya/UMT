import { birthday } from './birthday';
import { dayOfWeek } from './dayOfWeek';
import { isBrowser } from './isBrowser';
import { isNode } from './isNode';
import { isNodeWebkit } from './isNodeWebkit';
import { pipeFunction } from './pipeFunction';

export { birthday, dayOfWeek, pipeFunction };

export class UMTToolClass {
    #Localbirthday: typeof birthday;
    #LocaldayOfWeek: typeof dayOfWeek;
    #LocalpipeFunction: typeof pipeFunction;
    #LocalisBrowser: boolean;
    #LocalisNode: boolean;
    #LocalisNodeWebkit: boolean;
    constructor() {
        this.#Localbirthday = birthday;
        this.#LocaldayOfWeek = dayOfWeek;
        this.#LocalpipeFunction = pipeFunction;
        this.#LocalisBrowser = isBrowser;
        this.#LocalisNode = isNode;
        this.#LocalisNodeWebkit = isNodeWebkit;
    }
    get birthday() {
        return this.#Localbirthday;
    }
    get dayOfWeek() {
        return this.#LocaldayOfWeek;
    }
    get pipeFunction() {
        return this.#LocalpipeFunction;
    }
    get isBrowser() {
        return this.#LocalisBrowser;
    }
    get isNode() {
        return this.#LocalisNode;
    }
    get isNodeWebkit() {
        return this.#LocalisNodeWebkit;
    }
}

export const UMT_Tool = new UMTToolClass();
