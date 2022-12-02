import { birthdaySimple } from './birthday';
import { dayOfWeekSimple } from './dayOfWeekSimple';
import { deviationValueSimple } from './deviationValueSimple';

export class UMTSimpleToolClass {
    #Local_birthday: typeof birthdaySimple;
    #Local_dayOfWeek: typeof dayOfWeekSimple;
    #Local_deviationValue: typeof deviationValueSimple;
    constructor() {
        this.#Local_dayOfWeek = dayOfWeekSimple;
        this.#Local_deviationValue = deviationValueSimple;
        this.#Local_birthday = birthdaySimple;
    }
    get birthday() {
        return this.#Local_birthday;
    }
    get dayOfWeek() {
        return this.#Local_dayOfWeek;
    }
    get deviationValue() {
        return this.#Local_deviationValue;
    }
}
