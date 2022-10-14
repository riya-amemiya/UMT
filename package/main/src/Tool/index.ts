import { birthday } from './birthday';
import { dayOfWeek } from './dayOfWeek';
import { isBrowser } from './isBrowser';
import { isNode } from './isNode';
import { isNodeWebkit } from './isNodeWebkit';
import { pipeFunction } from './pipeFunction';

export { birthday, dayOfWeek, pipeFunction };

export class UMTToolClass {
    birthday: typeof birthday;
    dayOfWeek: typeof dayOfWeek;
    pipeFunction: typeof pipeFunction;
    isBrowser: boolean;
    isNode: boolean;
    isNodeWebkit: boolean;
    constructor() {
        this.birthday = birthday;
        this.dayOfWeek = dayOfWeek;
        this.pipeFunction = pipeFunction;
        this.isBrowser = isBrowser;
        this.isNode = isNode;
        this.isNodeWebkit = isNodeWebkit;
    }
}

export const UMT_Tool = new UMTToolClass();
