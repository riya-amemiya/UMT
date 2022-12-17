import { now } from './now';

export { now };

export class UMTDateClass {
    private localNow: typeof now;
    constructor() {
        this.localNow = now;
    }
    get now() {
        return this.localNow;
    }
}

export const UMT_Date = new UMTDateClass();
