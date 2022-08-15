import calculatorCore from './core';
import literalExpression from './literalExpression';

/**
 * 電卓
 * ()や符号に対応
 * 一文字までの方程式に対応
 * @param  {string} x
 * @param  {object} ex 為替
 */

const calculator = <T extends object>(x: string, ex?: T) => {
    x = x.replace(/\s+/g, '');
    if (x.indexOf('=') != -1) {
        return literalExpression(x);
    } else {
        return calculatorCore(x, ex);
    }
};
export default calculator;
