import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";
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
      sanitizedExpression = resolveParentheses(sanitizedExpression);
    }

    // 乗算、除算、べき乗の処理
    else if (containsMulDivExp(sanitizedExpression)) {
      sanitizedExpression = resolveMulDivExp(sanitizedExpression);
    }

    // 加算と減算の処理
    else if (containsAddSub(sanitizedExpression)) {
      sanitizedExpression = resolveAddSub(sanitizedExpression);
    }

    // もう計算するものがなければ結果を返す
    else {
      return sanitizedExpression;
    }
  }
};

const sanitizeSigns = (expr: string): string => {
  return expr
    .replace(/--/g, "+")
    .replace(/\+\+/g, "+")
    .replace(/\+-/g, "+0-")
    .replace(/-\+/g, "+0-");
};

const applyCurrencyExchange = <T extends { [key: string]: string | number }>(
  expr: string,
  rates: T,
): string => {
  let returnExpr = expr;
  // 通貨の交換ロジック
  for (const i in rates) {
    if (returnExpr.indexOf(i) !== -1) {
      const $ = returnExpr.match(new RegExp(`\\${i}([0-9]+)`));
      if ($) {
        returnExpr = returnExpr.replace($[0], convertCurrency($[0], rates));
      }
    }
  }
  return returnExpr;
};

const containsParentheses = (expr: string): boolean => {
  return expr.indexOf("(") !== -1 || expr.indexOf(")") !== -1;
};

const resolveParentheses = (expr: string): string => {
  // 括弧内の計算ロジック
  const match = expr.match(/\(\d+\.?(\d+)?(\*|\/|\+|-)\d+\.?(\d+)?\)/);
  if (match) {
    return expr.replace(
      match[0],
      calculatorCore(match[0].replace(/\(|\)/g, "")),
    );
  }
  return expr;
};

const containsMulDivExp = (expr: string): boolean => {
  return (
    expr.indexOf("^") !== -1 ||
    expr.indexOf("*") !== -1 ||
    expr.indexOf("/") !== -1
  );
};

const resolveMulDivExp = (expr: string): string => {
  // 乗算、除算、べき乗の計算ロジック
  const match = expr.match(/\d+\.?(\d+)?(\*|\/|\^)\d+\.?(\d+)?/);
  if (match) {
    const operands = match[0].split(/(\*|\/|\^)/);
    const result =
      operands[1] === "^"
        ? Number(operands[0]) ** Number(operands[2])
        : operands[1] === "*"
        ? multiplication(Number(operands[0]), Number(operands[2]))
        : operands[1] === "/"
        ? division(Number(operands[0]), Number(operands[2]))
        : 0;
    return expr.replace(match[0], String(result));
  }
  return expr;
};

const containsAddSub = (expr: string): boolean => {
  return expr.indexOf("+") !== -1 || expr.indexOf("-") !== -1;
};

const resolveAddSub = (expr: string): string => {
  // 加算、減算の計算ロジック
  const match = expr.match(/\d+\.?(\d+)?(\+|-)\d+\.?(\d+)?/);
  if (match) {
    const operands = match[0].split(/(\+|-)/);
    const result =
      operands[1] === "+"
        ? addition(Number(operands[0]), Number(operands[2]))
        : operands[1] === "-"
        ? subtract(Number(operands[0]), Number(operands[2]))
        : 0;
    return expr.replace(match[0], String(result));
  }
  return expr;
};
