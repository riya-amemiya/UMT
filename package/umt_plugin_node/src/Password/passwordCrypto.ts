import crypto from 'crypto';
export class PasswordCryptoClass {
    #Local_algorithm: string;
    constructor(algorithm: string) {
        const list = crypto.getCiphers();
        if (list.includes(algorithm)) {
            this.#Local_algorithm = algorithm;
        } else {
            throw new Error('Invalid algorithm');
        }
    }
    set setAlgorithm(algorithm: string) {
        this.#Local_algorithm = algorithm;
    }
    generator(text: string, password: crypto.CipherKey) {
        const iv = crypto.randomBytes(16);
        const cipher = crypto.createCipheriv(
            this.#Local_algorithm,
            password,
            iv,
        );
        let encrypted = cipher.update(text, 'utf8', 'hex');
        encrypted += cipher.final('hex');
        return {
            iv: iv.toString('hex'),
            encrypted,
        };
    }
    decrypt(text: string, key: crypto.CipherKey, iv: string) {
        const decipher = crypto.createDecipheriv(
            this.#Local_algorithm,
            key,
            Buffer.from(iv, 'hex'),
        );
        let decrypted = decipher.update(text, 'hex', 'utf8');
        decrypted += decipher.final('utf8');
        return decrypted;
    }
}
