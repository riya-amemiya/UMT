/**
 * 数字かどうか
 * @param  {any} x
 * @param  {boolean} loose 文字列も対象にするかどうか
 * @returns boolean
 */
export const isNumber = (x: any, loose: boolean = true) => {
    return x !== null && typeof x !== 'boolean' && loose
        ? isFinite(x)
        : Number.isFinite(x);
};
