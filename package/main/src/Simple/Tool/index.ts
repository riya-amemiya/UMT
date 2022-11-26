import { birthdaySimple } from './birthday';

export class UMTSimpleToolClass {
    #Local_birthday: typeof birthdaySimple;
    constructor() {
        this.#Local_birthday = birthdaySimple;
    }
    get birthday() {
        return this.#Local_birthday;
    }
}
