import ValueSwap from '../ValueSwap';

/**
 * 自然数の最大公約数
 * @param  {number} x
 * @param  {number} y
 * @param  {number} ...z
 */
const EuclideanAlgorithm = (x: number, y: number, ...z: number[]) => {
    if (x === 0 || y === 0) return 0;
    [x, y] = ValueSwap(x, y);
    /* ユークリッドの互除法 */
    let r = y % x;
    while (r !== 0) {
        y = x;
        x = r;
        r = y % x;
    }
    if (z.length > 0) {
        for (let i = 0; i < z.length; i++) {
            x = EuclideanAlgorithm(x, z[i]);
        }
    }
    return x;
};
export default EuclideanAlgorithm;
