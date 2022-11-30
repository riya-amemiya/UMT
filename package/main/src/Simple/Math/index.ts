import { dayOfWeekSimple } from './dayOfWeek';
import { deviationValueSimple } from './deviationValue';
export class UMTSimpleMathClass {
    #Local_dayOfWeek: typeof dayOfWeekSimple;
    #Local_deviationValue: typeof deviationValueSimple;
    constructor() {
        this.#Local_dayOfWeek = dayOfWeekSimple;
        this.#Local_deviationValue = deviationValueSimple;
    }
    get dayOfWeek() {
        return this.#Local_dayOfWeek;
    }
    get deviationValue() {
        return this.#Local_deviationValue;
    }
}
