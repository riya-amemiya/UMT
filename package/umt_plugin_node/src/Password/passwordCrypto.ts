import crypto from 'crypto';
export class PasswordCryptoClass {
    #Local_algorithm: string;
    #Local_iv: string;
    constructor(algorithm: string) {
        const list = crypto.getCiphers();
        if (list.includes(algorithm)) {
            this.#Local_algorithm = algorithm;
        } else {
            throw new Error('Invalid algorithm');
        }
        this.#Local_iv = '';
    }
    set setAlgorithm(algorithm: string) {
        this.#Local_algorithm = algorithm;
    }
    get getIv() {
        return this.#Local_iv;
    }
    reset() {
        this.#Local_iv = '';
    }
    generator(text: string, key: crypto.CipherKey) {
        this.#Local_iv = crypto.randomBytes(16).toString('hex');
        const cipher = crypto.createCipheriv(
            this.#Local_algorithm,
            key,
            Buffer.from(this.#Local_iv, 'hex'),
        );
        let encrypted = cipher.update(text, 'utf8', 'hex');
        encrypted += cipher.final('hex');
        return {
            encrypted,
        };
    }
    decrypt(text: string, key: crypto.CipherKey) {
        const decipher = crypto.createDecipheriv(
            this.#Local_algorithm,
            key,
            Buffer.from(this.#Local_iv, 'hex'),
        );
        let decrypted = decipher.update(text, 'hex', 'utf8');
        decrypted += decipher.final('utf8');
        return decrypted;
    }
}
