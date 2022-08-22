import gcd from './gcd';
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
    return (x / gcd(x, y)) * y;
};
export default lcm;
