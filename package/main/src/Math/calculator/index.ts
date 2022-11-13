import { calculatorCore } from './core';
import { literalExpression } from './literalExpression';

/**
 * 電卓
 * ()や符号に対応
 * 一文字までの方程式に対応
 * @param  {string} x
 * @param  {object} exchange 為替
 */

export const calculator = <T extends object>(
    x: string,
    exchange?: T,
) => {
    x = x.replace(/\s+/g, ''); // Remove spaces
    if (x.indexOf('=') != -1) {
        return literalExpression(x); // If the expression contains an equal sign, then it is a literal expression
    } else {
        return calculatorCore(x, exchange);
    }
};
