import { nCr } from './nCr';
/**
 * 重複を許して取り出す
 * @param  {number} n
 * @param  {number} r
 */
const nHr = (n: number, r: number) => {
    if (n === 0 || r === 0) {
        return NaN;
    }
    const y = nCr(n + r - 1, r);
    if (1 > y) return 0;
    return y;
};
export { nHr };
