import { calculatorCore } from "./core";
import { literalExpression } from "./literalExpression";

/**
 * 電卓
 * ()や符号に対応
 * 一文字までの方程式に対応
 * @param  {string} expression 計算式または方程式
 * @param  {object} exchange 為替
 * @returns 計算結果
 * @example calculator("1+2"); // 3
 */
export const calculator = <T extends Record<string, string | number>>(
  expression: string,
  exchange?: T,
) => {
  const cleanExpression = expression.replaceAll(/\s+/g, "");
  return cleanExpression.includes("=")
    ? literalExpression(cleanExpression)
    : calculatorCore(cleanExpression, exchange);
};
