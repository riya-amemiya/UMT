import { UMTNodeFile } from './File';
import { check } from './File/check';
import { read } from './File/read';
import { UMTNodePassword } from './Password';
import { PasswordCryptoClass } from './Password/passwordCrypto';
import { passwordGenerator } from './Password/passwordGenerator';

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
export { check, read, passwordGenerator, PasswordCryptoClass };
export const UMTNode = new UMTNodeClass();
