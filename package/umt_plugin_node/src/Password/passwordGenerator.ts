export interface passwordGeneratorOptions {
    length: number;
    numbers: boolean;
    uppercase: boolean;
    symbols: boolean;
}
export const passwordGenerator = ({
    length = 10,
    numbers = true,
    uppercase = true,
    symbols = true,
}: passwordGeneratorOptions) => {
    const lowercaseLetters = 'abcdefghijklmnopqrstuvwxyz';
    const numbersString = '0123456789';
    const symbolsStrig = '!@#$%^&*()_+-={}[]:";<>?,./|';
    const uppercaseLetters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';

    let charSet = lowercaseLetters;

    if (numbers) {
        charSet += numbersString;
    }

    if (symbols) {
        charSet += symbolsStrig;
    }

    if (uppercase) {
        charSet += uppercaseLetters;
    }

    let password = '';

    for (let i = 0; i < length; i++) {
        const randomPoz = Math.floor(Math.random() * charSet.length);

        password += charSet.substring(randomPoz, randomPoz + 1);
    }

    return password;
};
