import { birthdaySimple } from './birthday';

export class UMTSimpleToolClass {
    #Localbirthday: typeof birthdaySimple;
    constructor() {
        this.#Localbirthday = birthdaySimple;
    }
    get birthday() {
        return this.#Localbirthday;
    }
}
