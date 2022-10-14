import { birthday } from './birthday';
import { dayOfWeek } from './dayOfWeek';
import { pipeFunction } from './pipeFunction';

export { birthday, dayOfWeek, pipeFunction };

export class UMTToolClass {
    birthday: typeof birthday;
    dayOfWeek: typeof dayOfWeek;
    pipeFunction: typeof pipeFunction;
    constructor() {
        this.birthday = birthday;
        this.dayOfWeek = dayOfWeek;
        this.pipeFunction = pipeFunction;
    }
}

export const UMT_Tool = new UMTToolClass();
