import { UMTNodeFile } from './File';
import { UMTNodePassword } from './Password';

export class UMTNodeClass {
    #Local_File: typeof UMTNodeFile;
    #Local_Password: typeof UMTNodePassword;
    constructor() {
        this.#Local_File = UMTNodeFile;
        this.#Local_Password = UMTNodePassword;
    }
    get File() {
        return this.#Local_File;
    }
    get Password() {
        return this.#Local_Password;
    }
}
export const UMTNode = new UMTNodeClass();
