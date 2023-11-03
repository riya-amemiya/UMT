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
  let copyX = x;
  copyX = copyX.replaceAll(/\s+/g, "");
  if (copyX.includes("=")) {
    return literalExpression(copyX);
  }
  return calculatorCore(copyX, exchange);
};
