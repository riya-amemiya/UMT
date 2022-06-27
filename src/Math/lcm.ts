import euclideanAlgorithm from './euclideanAlgorithm';
import valueSwap from './valueSwap';
/**
 * 最小公倍数
 * @param  {number} x
 * @param  {number} y
 */
const lcm = (x: number, y: number) => {
    if (x === 0 || y === 0) {
        return 0;
    }
    [x, y] = valueSwap(x, y);
    return (x / euclideanAlgorithm(x, y)) * y;
};
export default lcm;
