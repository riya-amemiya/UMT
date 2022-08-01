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
            cache[1] = i;
        } else {
            cache[0] = i
                .split(/([0-9]+[a-zA-Z]+)|([^a-zA-Z]+)/)
                .filter((n) => n !== '' && typeof n != 'undefined');
        }
    }
    if (cache[0][1]) {
        if (cache[0][1].indexOf('+') !== -1) {
            cache[0][1] = cache[0][1].replace(/\+/g, 'plus');
        }
        if (cache[0][1].indexOf('-') !== -1) {
            cache[0][1] = cache[0][1].replace(/\-/g, 'minus');
        }
        if (cache[0][1].indexOf('plus') !== 1) {
            cache[0][1] = cache[0][1].replace(/plus/g, '-');
        }
        if (cache[0][1].indexOf('minus') !== 1) {
            cache[0][1] = cache[0][1].replace(/minus/g, '+');
        }
    }

    cache[1] = cache[0][1]
        ? calculatorCore(`${cache[1]}${cache[0][1]}`)
        : calculatorCore(cache[1]);

    cache[0] = cache[0][0]
        .split(/([0-9]+)|([a-zA-Z]+)/)
        .filter((n) => n !== '' && typeof n != 'undefined');

    const cacheGcd = gcd(Number(cache[0][0]), Number(cache[1]));
    if (cacheGcd !== 1) {
        return `${division(Number(cache[1]), cacheGcd)[0]}/${
            division(Number(cache[0][0]), cacheGcd)[0]
        }`;
    }
    return `${Number(cache[1])}/${Number(cache[0][0])}`;
};
export default literalExpression;
