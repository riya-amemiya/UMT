import { nowSimple } from './now';

export class UMTSimpleDateClass {
    private localNow: typeof nowSimple;
    constructor() {
        this.localNow = nowSimple;
    }
    get now() {
        return this.localNow;
    }
}
