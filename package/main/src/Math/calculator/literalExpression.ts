import { calculatorCore } from "./core";

import { division } from "@/Math/division";
import { gcd } from "@/Math/gcd";
/**
 * 文字式の方程式を計算する
 * @param x 方程式の文字列
 * @returns 計算結果
 * @example literalExpression("x+1=2"); // "1"
 */
export const literalExpression = (x: string): string => {
  // 方程式の数値と変数部分を格納
  let numericalPart = "";
  let variablePart: string[] = [];

  // 等号で分割し、数値と変数部分を識別
  for (const part of x.split("=")) {
    if (/[A-Za-z]+/.test(part)) {
      variablePart = part
        .split(/(\d+[A-Za-z]+)|([^A-Za-z]+)/)
        .filter((n) => n && n !== undefined);
    } else {
      numericalPart = part;
    }
  }

  // 変数部分の計算
  if (variablePart[1]) {
    variablePart[1] = calculatorCore(variablePart[1])
      .replaceAll("+", "plus")
      .replaceAll("-", "minus")
      .replaceAll("plus", "-")
      .replaceAll("minus", "+");
  }

  // 数値部分の計算
  numericalPart = variablePart[1]
    ? calculatorCore(`${numericalPart}${variablePart[1]}`)
    : calculatorCore(numericalPart);

  // 変数部分を再分割
  variablePart = variablePart[0]
    .split(/(\d+)|([A-Za-z]+)/)
    .filter((n) => n && n !== undefined);

  // 数値がない場合は計算結果を返す
  if (Number.isNaN(Number(variablePart[0]))) {
    return numericalPart;
  }

  // 最大公約数で簡約
  const commonGcd = gcd(Number(variablePart[0]), Number(numericalPart));
  if (commonGcd !== 1) {
    numericalPart = `${division(Number(numericalPart), commonGcd)}/${division(
      Number(variablePart[0]),
      commonGcd,
    )}`;
    if (/(-?)\d+\/1/.test(numericalPart)) {
      return numericalPart.replace(/\/1/, "");
    }
  }
  if (variablePart[0] !== "1") {
    numericalPart = `${numericalPart}/${variablePart[0]}`;
  }

  return numericalPart;
};
