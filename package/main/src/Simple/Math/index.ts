import { dayOfWeekSimple } from './dayOfWeek';
import { deviationValueSimple } from './deviationValue';
export class UMTSimpleMathClass {
    #LocaldayOfWeek: typeof dayOfWeekSimple;
    #LocaldeviationValue: typeof deviationValueSimple;
    constructor() {
        this.#LocaldayOfWeek = dayOfWeekSimple;
        this.#LocaldeviationValue = deviationValueSimple;
    }
    get dayOfWeek() {
        return this.#LocaldayOfWeek;
    }
    get deviationValue() {
        return this.#LocaldeviationValue;
    }
}
