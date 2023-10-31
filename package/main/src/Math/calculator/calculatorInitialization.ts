import { calculator } from "./calculator";

/**
 * 電卓関数を初期化する
 * @param {object} exchange - 為替レート
 * @return {Function} - calculator
 */
export const calculatorInitialization = <
  T extends { [key: string]: string | number },
>(
  exchange: T,
): ((x: string) => string) => {
  /**
   * @param {string} x - 金額
   * @return {string} - 計算結果
   */
  return (x: string): string => calculator(x, exchange);
};
