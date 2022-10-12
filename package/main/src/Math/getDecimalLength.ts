/**
 * 小数点以下の桁数
 * @param  {number} value
 */
export const getDecimalLength = (value: number) => {
    let x = (value + '').split('.')[1];
    if (typeof x !== 'undefined' && x.length > 0) {
        return x.length;
    }
    return 0;
};
