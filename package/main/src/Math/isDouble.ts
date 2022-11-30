/**
 * @param {any} x
 * @param {boolean} [loose=true] - 文字列も対象にするかどうか
 * @returns boolean
 */
export const isDouble = (x: any, loose: boolean = true) => {
    if (loose) {
        return (
            isFinite(x) &&
            !Number.isNaN(x) &&
            Number.isFinite(Number(x)) &&
            !Number.isInteger(Number(x))
        );
    } else {
        return (
            x !== null && typeof x !== 'boolean' && Number.isFinite(x)
        );
    }
};
