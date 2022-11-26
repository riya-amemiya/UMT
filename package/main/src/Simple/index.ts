import { UMTSimpleDateClass } from './Date/index';
import { UMTSimpleMathClass } from './Math';
import { UMTSimpleToolClass } from './Tool';
export class UMTSimpleClass<LOCALDATE, LOCALMATH, LOCALTOOL> {
    #Local_Date: LOCALDATE;
    #Local_Math: LOCALMATH;
    #Local_Tool: LOCALTOOL;

    constructor(
        constructorLocalDateValue: LOCALDATE,
        constructorLocalMathValue: LOCALMATH,
        constructorLocalToolValue: LOCALTOOL,
    ) {
        this.#Local_Date = constructorLocalDateValue;
        this.#Local_Math = constructorLocalMathValue;
        this.#Local_Tool = constructorLocalToolValue;
    }
    get Date() {
        return this.#Local_Date;
    }
    get Math() {
        return this.#Local_Math;
    }
    get Tool() {
        return this.#Local_Tool;
    }
}

export const UMT_Simple = new UMTSimpleClass(
    new UMTSimpleDateClass(),
    new UMTSimpleMathClass(),
    new UMTSimpleToolClass(),
);
