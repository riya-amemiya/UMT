import { now } from './now';

export { now };

export class UMTDateClass {
    #Local_now: typeof now;
    constructor() {
        this.#Local_now = now;
    }
    get now() {
        return this.#Local_now;
    }
}

export const UMT_Date = new UMTDateClass();
