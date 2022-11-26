import { nowSimple } from './now';

export class UMTSimpleDateClass {
    #Local_now: typeof nowSimple;
    constructor() {
        this.#Local_now = nowSimple;
    }
    get now() {
        return this.#Local_now;
    }
}
