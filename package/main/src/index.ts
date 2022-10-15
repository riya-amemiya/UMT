import { UMT_Array } from './Array';
import { UMT_Date } from './Date';
import { UMT_Math } from './Math';
import { UMT_Simple } from './Simple';
import { UMT_Tool } from './Tool';
export class UMTClass {
    #LocalArray: typeof UMT_Array;
    #LocalDate: typeof UMT_Date;
    #LocalMath: typeof UMT_Math;
    #LocalSimple: typeof UMT_Simple;
    #LocalTool: typeof UMT_Tool;
    constructor() {
        this.#LocalArray = UMT_Array;
        this.#LocalDate = UMT_Date;
        this.#LocalMath = UMT_Math;
        this.#LocalSimple = UMT_Simple;
        this.#LocalTool = UMT_Tool;
    }
    get Array() {
        return this.#LocalArray;
    }
    get Date() {
        return this.#LocalDate;
    }
    get Math() {
        return this.#LocalMath;
    }
    get Simple() {
        return this.#LocalSimple;
    }
    get Tool() {
        return this.#LocalTool;
    }
}
export const UMT = new UMTClass();
