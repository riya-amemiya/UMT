import { birthday } from './birthday';
import { dayOfWeek } from './dayOfWeek';
import { isBrowser } from './isBrowser';
import { isNode } from './isNode';
import { isNodeWebkit } from './isNodeWebkit';
import { pipeFunction } from './pipeFunction';

export { birthday, dayOfWeek, pipeFunction };

export class UMTToolClass {
    private localBirthday: typeof birthday;
    private localDayOfWeek: typeof dayOfWeek;
    private localPipeFunction: typeof pipeFunction;
    private localIsBrowser: boolean;
    private localIsNode: boolean;
    private localIsNodeWebkit: boolean;
    constructor() {
        this.localBirthday = birthday;
        this.localDayOfWeek = dayOfWeek;
        this.localPipeFunction = pipeFunction;
        this.localIsBrowser = isBrowser;
        this.localIsNode = isNode;
        this.localIsNodeWebkit = isNodeWebkit;
    }
    get birthday() {
        return this.localBirthday;
    }
    get dayOfWeek() {
        return this.localDayOfWeek;
    }
    get pipeFunction() {
        return this.localPipeFunction;
    }
    get isBrowser() {
        return this.localIsBrowser;
    }
    get isNode() {
        return this.localIsNode;
    }
    get isNodeWebkit() {
        return this.localIsNodeWebkit;
    }
}

export const UMT_Tool = new UMTToolClass();
