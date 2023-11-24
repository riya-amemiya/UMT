import { calculatorCore } from "./core";
import { literalExpression } from "./literalExpression";

/**
 * 電卓
 * ()や符号に対応
 * 一文字までの方程式に対応
 * @param  {string} x
 * @param  {object} exchange 為替
 * @returns 計算結果
 * @example calculator("1+2"); // 3
 */
export const calculator = <T extends { [key: string]: string | number }>(
  x: string,
  exchange?: T,
) => {
  x = x.replaceAll(/\s+/g, "");
  if (x.includes("=")) {
    return literalExpression(x);
  }
  return calculatorCore(x, exchange);
};
