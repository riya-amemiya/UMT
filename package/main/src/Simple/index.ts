import { nowSimple } from './Date/now';
import { dayOfWeekSimple } from './Math/dayOfWeek';
import { deviationValueSimple } from './Math/deviationValue';
import { birthdaySimple } from './Tool/birthday';

export class UMTSimpleClass {
    Date: {
        nowSimple: typeof nowSimple;
    };
    Math: {
        dayOfWeekSimple: typeof dayOfWeekSimple;
        deviationValueSimple: typeof deviationValueSimple;
    };
    Tool: {
        birthdaySimple: typeof birthdaySimple;
    };

    constructor() {
        this.Date = {
            nowSimple: nowSimple,
        };
        this.Math = {
            dayOfWeekSimple: dayOfWeekSimple,
            deviationValueSimple: deviationValueSimple,
        };

        this.Tool = {
            birthdaySimple: birthdaySimple,
        };
    }
}

export const UMT_Simple = new UMTSimpleClass();
