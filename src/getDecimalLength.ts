/**
 * 小数点以下の桁
 * @param  {number} value
 */
const getDecimalLength = (value: number) => {
    let x = (value + '').split('.')[1];
    if (typeof x !== 'undefined' && x.length > 0) {
        return x.length;
    }
    return 0;
};
export default getDecimalLength;
