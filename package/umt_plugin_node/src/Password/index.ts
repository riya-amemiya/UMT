import { PasswordCryptoClass } from './passwordCrypto';
import { passwordGenerator } from './passwordGenerator';

export class UMTNodePasswordClass {
    #Local_passwordGenerator: typeof passwordGenerator;
    #Local_PasswordCryptoClass: typeof PasswordCryptoClass;
    constructor() {
        this.#Local_passwordGenerator = passwordGenerator;
        this.#Local_PasswordCryptoClass = PasswordCryptoClass;
    }
    get passwordGenerator() {
        return this.#Local_passwordGenerator;
    }
    get PasswordCryptoClass() {
        return this.#Local_PasswordCryptoClass;
    }
}
export { passwordGenerator, PasswordCryptoClass };

export const UMTNodePassword = new UMTNodePasswordClass();
