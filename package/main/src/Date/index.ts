import { now } from './now';

export { now };

export class UMTDateClass {
    #Localnow: typeof now;
    constructor() {
        this.#Localnow = now;
    }
    get now() {
        return this.#Localnow;
    }
}

export const UMT_Date = new UMTDateClass();
