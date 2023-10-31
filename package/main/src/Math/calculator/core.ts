import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";
import { isNumber } from "../isNumber";
import { convertCurrency } from "./convertCurrency";

export const calculatorCore = <T extends { [key: string]: string | number }>(
  expression: string,
  currencyExchange?: T,
): string => {
  let sanitizedExpression = expression;

  // 符号の処理
  sanitizedExpression = sanitizeSigns(sanitizedExpression);

  // 主要な計算ループ
  while (true) {
    // 通貨計算
    if (currencyExchange) {
      sanitizedExpression = applyCurrencyExchange(
        sanitizedExpression,
        currencyExchange,
      );
    }

    // 括弧の処理
    if (containsParentheses(sanitizedExpression)) {
      const temporary = resolveParentheses(sanitizedExpression);
      if (temporary === NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // 乗算、除算、べき乗の処理
    else if (containsMulDivExp(sanitizedExpression)) {
      const temporary = resolveMulDivExp(sanitizedExpression);
      if (temporary === NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // 加算と減算の処理
    else if (
      containsAddSub(sanitizedExpression) &&
      !isNumber(sanitizedExpression)
    ) {
      const temporary = resolveAddSub(sanitizedExpression);
      if (temporary === NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // もう計算するものがなければ結果を返す
    else {
      return sanitizedExpression;
    }
  }
};

const sanitizeSigns = (expr: string): string => {
  return expr
    .replaceAll("--", "+")
    .replaceAll("++", "+")
    .replaceAll("+-", "+0-")
    .replaceAll("-+", "+0-");
};

const applyCurrencyExchange = <T extends { [key: string]: string | number }>(
  expr: string,
  rates: T,
): string => {
  let returnExpr = expr;
  // 通貨の交換ロジック
  for (const index in rates) {
    if (returnExpr.includes(index)) {
      const $ = returnExpr.match(new RegExp(`\\${index}([0-9]+)`));
      if ($) {
        returnExpr = returnExpr.replace($[0], convertCurrency($[0], rates));
      }
    }
  }
  return returnExpr;
};

const containsParentheses = (expr: string): boolean => {
  return expr.includes("(") || expr.includes(")");
};

const resolveParentheses = (expr: string): string => {
  // 括弧内の計算ロジック
  const match = expr.match(/\(\d+\.?(\d+)?([*+/-])\d+\.?(\d+)?\)/);
  if (match) {
    return expr.replace(
      match[0],
      calculatorCore(match[0].replaceAll(/\(|\)/g, "")),
    );
  }
  return NaN.toString();
};

const containsMulDivExp = (expr: string): boolean => {
  return expr.includes("^") || expr.includes("*") || expr.includes("/");
};

const resolveMulDivExp = (expr: string): string => {
  // 乗算、除算、べき乗の計算ロジック
  const match = expr.match(/\d+\.?(\d+)?([*/^])\d+\.?(\d+)?/);
  if (match) {
    const operands = match[0].split(/([*/^])/);
    const result =
      operands[1] === "^"
        ? /^\d+\.?(\d+)?([*/^])\d+\.?(\d+)?$/.test(expr)
          ? Number(operands[0]) ** Number(operands[2])
          : Number(operands[2]) ** Number(operands[0])
        : operands[1] === "*"
        ? multiplication(Number(operands[0]), Number(operands[2]))
        : operands[1] === "/"
        ? division(Number(operands[0]), Number(operands[2]))
        : 0;
    return expr.replace(match[0], String(result));
  }
  return NaN.toString();
};

const containsAddSub = (expr: string): boolean => {
  return expr.includes("+") || expr.includes("-");
};

const resolveAddSub = (expr: string): string => {
  // 加算、減算の計算ロジック
  const match = expr.match(/(-?\d+)\.?(\d+)?(\+|-)(-?\d+)\.?(\d+)?/);
  if (match) {
    const result =
      match[3] === "+"
        ? addition(Number(match[1]), Number(match[4]))
        : match[3] === "-"
        ? subtract(Number(match[1]), Number(match[4]))
        : 0;
    return expr.replace(match[0], String(result));
  }
  return NaN.toString();
};
