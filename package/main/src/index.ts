import { UMT_Array } from './Array';
import { UMT_Date } from './Date';
import { UMT_Math } from './Math';
import { UMT_Simple } from './Simple';
import { UMT_Tool } from './Tool';
export class UMTClass {
    #Local_Array: typeof UMT_Array;
    #Local_Date: typeof UMT_Date;
    #Local_Math: typeof UMT_Math;
    #Local_Simple: typeof UMT_Simple;
    #Local_Tool: typeof UMT_Tool;
    constructor() {
        this.#Local_Array = UMT_Array;
        this.#Local_Date = UMT_Date;
        this.#Local_Math = UMT_Math;
        this.#Local_Simple = UMT_Simple;
        this.#Local_Tool = UMT_Tool;
    }
    get Array() {
        return this.#Local_Array;
    }
    get Date() {
        return this.#Local_Date;
    }
    get Math() {
        return this.#Local_Math;
    }
    get Simple() {
        return this.#Local_Simple;
    }
    get Tool() {
        return this.#Local_Tool;
    }
}
export const UMT = new UMTClass();
