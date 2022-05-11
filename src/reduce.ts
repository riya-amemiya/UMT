import euclideanAlgorithm from './euclideanAlgorithm';
/**
 * 約分
 * @param  {number} x
 * @param  {number} y
 */
const reduce = (x: number, y: number) => {
    if (x === 0 || y === 0) {
        return { x: NaN, y: NaN };
    }
    let n = euclideanAlgorithm(x, y);
    return { x: x / n, y: y / n, gcd: euclideanAlgorithm(x, y) };
};
export default reduce;
