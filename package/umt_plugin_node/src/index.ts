import { UMTNodeFile } from './File';

export class UMTNodeClass {
    #Local_File: typeof UMTNodeFile;
    constructor() {
        this.#Local_File = UMTNodeFile;
    }
    get File() {
        return this.#Local_File;
    }
}
export const UMTNode = new UMTNodeClass();
