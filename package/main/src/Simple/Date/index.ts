import { nowSimple } from './now';

export class UMTSimpleDateClass {
    #Localnow: typeof nowSimple;
    constructor() {
        this.#Localnow = nowSimple;
    }
    get now() {
        return this.#Localnow;
    }
}
