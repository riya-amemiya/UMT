import division from '../division';
import gcd from '../gcd';
import calculatorCore from './core';
/**
 * @param  {string} x
 */
const literalExpression = (x: string) => {
    let cache: [string[], string] = [[], ''];
    for (const i of x.split('=')) {
        if (/[a-zA-Z]+/.test(i) === false) {
            cache[1] = calculatorCore(i);
        } else {
            cache[0] = i.split(/([a-zA-Z])/).filter((n) => n !== '');
        }
    }
    const cacheGcd = gcd(Number(cache[0][0]), Number(cache[1]));
    if (cacheGcd !== 1) {
        return `${division(Number(cache[1]), cacheGcd)[0]}/${
            division(Number(cache[0][0]), cacheGcd)[0]
        }`;
    }
    return `${Number(cache[1])}/${Number(cache[0][0])}`;
};
export default literalExpression;
