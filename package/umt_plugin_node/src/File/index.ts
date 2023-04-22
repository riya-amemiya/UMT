import { check } from './check';
import { read } from './read';

export class UMTNodeFileClass {
    #Local_check: typeof check;
    #Local_read: typeof read;
    constructor() {
        this.#Local_check = check;
        this.#Local_read = read;
    }
    get check() {
        return this.#Local_check;
    }
    get read() {
        return this.#Local_read;
    }
}
export { check, read };
export const UMTNodeFile = new UMTNodeFileClass();
