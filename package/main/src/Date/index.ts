import { now } from './now';

export { now };

export class UMTDateClass {
    now: typeof now;
    constructor() {
        this.now = now;
    }
}

export const UMT_Date = new UMTDateClass();
