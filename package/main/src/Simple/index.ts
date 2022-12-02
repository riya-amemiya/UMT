import { UMTSimpleDateClass } from './Date/index';
import { UMTSimpleToolClass } from './Tool';
export class UMTSimpleClass<LOCALDATE, LOCALTOOL> {
    #Local_Date: LOCALDATE;
    #Local_Tool: LOCALTOOL;

    constructor(
        constructorLocalDateValue: LOCALDATE,
        constructorLocalToolValue: LOCALTOOL,
    ) {
        this.#Local_Date = constructorLocalDateValue;
        this.#Local_Tool = constructorLocalToolValue;
    }
    get Date() {
        return this.#Local_Date;
    }
    get Tool() {
        return this.#Local_Tool;
    }
}

export const UMT_Simple = new UMTSimpleClass(
    new UMTSimpleDateClass(),
    new UMTSimpleToolClass(),
);
