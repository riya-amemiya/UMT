/**
 * 倍数
 * @param  {number} x
 * @param  {number} n
 */
const multiples = (x: number, n: number) => {
    let result = [];
    for (let i = 1; i <= n; i++) {
        result.push(x * i);
    }
    return result;
};
export default multiples;
