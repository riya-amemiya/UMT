import { calculatorCore } from "./core";
import { literalExpression } from "./literalExpression";

/**
 * 電卓
 * ()や符号に対応
 * 一文字までの方程式に対応
 * @param  {string} x
 * @param  {object} exchange 為替
 */

export const calculator = <T extends { [key: string]: string | number }>(
  x: string,
  exchange?: T,
) => {
  let copyX = x;
  copyX = copyX.replaceAll(/\s+/g, ""); // Remove spaces
  if (copyX.includes("=")) {
    return literalExpression(copyX); // If the expression contains an equal sign, then it is a literal expression
  }
  return calculatorCore(copyX, exchange);
};
