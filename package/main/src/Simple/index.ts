import { UMTSimpleDateClass } from './Date/index';
import { UMTSimpleMathClass } from './Math';
import { UMTSimpleToolClass } from './Tool';
export class UMTSimpleClass<LOCALDATE, LOCALMATH, LOCALTOOL> {
    #LocalDate: LOCALDATE;
    #LocalMath: LOCALMATH;
    #LocalTool: LOCALTOOL;

    constructor(
        constructorLocalDateValue: LOCALDATE,
        constructorLocalMathValue: LOCALMATH,
        constructorLocalToolValue: LOCALTOOL,
    ) {
        this.#LocalDate = constructorLocalDateValue;
        this.#LocalMath = constructorLocalMathValue;
        this.#LocalTool = constructorLocalToolValue;
    }
    get Date() {
        return this.#LocalDate;
    }
    get Math() {
        return this.#LocalMath;
    }
    get Tool() {
        return this.#LocalTool;
    }
}

export const UMT_Simple = new UMTSimpleClass(
    new UMTSimpleDateClass(),
    new UMTSimpleMathClass(),
    new UMTSimpleToolClass(),
);
