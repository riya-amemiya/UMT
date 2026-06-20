import { convertCurrency } from "./convertCurrency";

import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";
import { escapeRegExp } from "@/Tool/escapeRegExp";
import { isNumber } from "@/Validate/isNumber";

export const calculatorCore = <T extends { [key: string]: string | number }>(
  expression: string,
  currencyExchange?: T,
): string => {
  // Handle empty string
  if (expression === "") {
    return "";
  }

  let sanitizedExpression = expression;

  // Handle signs
  sanitizedExpression = sanitizeSigns(sanitizedExpression);

  // Main calculation loop
  while (true) {
    // Currency conversion
    if (currencyExchange) {
      sanitizedExpression = applyCurrencyExchange(
        sanitizedExpression,
        currencyExchange,
      );
    }

    // Handle parentheses
    if (containsParentheses(sanitizedExpression)) {
      const temporary = resolveParentheses(sanitizedExpression);
      if (temporary === Number.NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // Handle multiplication and exponentiation
    else if (containsMulExp(sanitizedExpression)) {
      const temporary = resolveMulExp(sanitizedExpression);
      if (temporary === Number.NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // Handle division
    else if (containsDiv(sanitizedExpression)) {
      const temporary = resolveDiv(sanitizedExpression);
      if (temporary === Number.NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // Handle addition and subtraction
    else if (
      containsAddSub(sanitizedExpression) &&
      !isNumber(sanitizedExpression)
    ) {
      const temporary = resolveAddSub(sanitizedExpression);
      if (temporary === Number.NaN.toString()) {
        return sanitizedExpression;
      }
      sanitizedExpression = temporary;
    }

    // Return result if no more calculations needed
    else {
      const number_ = Number(sanitizedExpression);
      if (!Number.isNaN(number_)) {
        // Handle floating point precision issues
        const rounded = Math.round(number_ * 1e10) / 1e10;
        return rounded.toString();
      }
      return sanitizedExpression;
    }
  }
};

const sanitizeSigns = (expression: string): string => {
  return expression
    .replaceAll("--", "+")
    .replaceAll("++", "+")
    .replaceAll("+-", "+0-")
    .replaceAll("-+", "+0-");
};

const currencyRegexCache = new Map<string, RegExp>();

const getCurrencyRegex = (currencySymbol: string): RegExp => {
  const cached = currencyRegexCache.get(currencySymbol);
  if (cached) {
    return cached;
  }
  const regex = new RegExp(`${escapeRegExp(currencySymbol)}([0-9]+)`);
  currencyRegexCache.set(currencySymbol, regex);
  return regex;
};

const applyCurrencyExchange = <T extends { [key: string]: string | number }>(
  expression: string,
  rates: T,
): string => {
  let returnExpression = expression;
  // Currency exchange logic
  for (const index in rates) {
    if (!returnExpression.includes(index)) {
      continue;
    }

    const $ = getCurrencyRegex(index).exec(returnExpression);
    if ($) {
      returnExpression = returnExpression.replace(
        $[0],
        convertCurrency($[0], rates),
      );
    }
  }
  return returnExpression;
};

const containsParentheses = (expression: string): boolean => {
  return expression.includes("(") || expression.includes(")");
};

const resolveParentheses = (expression: string): string => {
  // Logic for calculations inside parentheses
  const match = /\((-?\d+(?:\.\d+)?)([*+/-])(-?\d+(?:\.\d+)?)\)/.exec(
    expression,
  );
  if (match) {
    return expression.replace(
      match[0],
      calculatorCore(match[0].replaceAll(/\(|\)/g, "")),
    );
  }
  return Number.NaN.toString();
};

const containsMulExp = (expression: string): boolean => {
  return expression.includes("^") || expression.includes("*");
};

const containsDiv = (expression: string): boolean => {
  return expression.includes("/");
};

const resolveMulExp = (expression: string): string => {
  // Logic for multiplication and exponentiation
  const match = /(.*?)(-?\d+(?:\.\d+)?)([*^])(-?\d+(?:\.\d+)?)$/.exec(
    expression,
  );
  if (match) {
    const result =
      match[3] === "^"
        ? Number(match[2]) ** Number(match[4])
        : multiplication(Number(match[2]), Number(match[4]));
    return `${match[1]}${result}`;
  }
  return Number.NaN.toString();
};

const resolveDiv = (expression: string): string => {
  // Logic for division
  const match = /(-?\d+(?:\.\d+)?)\/(-?\d+(?:\.\d+)?)/.exec(expression);
  if (match) {
    const result = division(Number(match[1]), Number(match[2]));
    return expression.replace(match[0], String(result));
  }
  return Number.NaN.toString();
};

const containsAddSub = (expression: string): boolean => {
  return expression.includes("+") || expression.includes("-");
};

const resolveAddSub = (expression: string): string => {
  // Logic for addition and subtraction
  const match = /(-?\d+(?:\.\d+)?)(\+|-)(-?\d+(?:\.\d+)?)/.exec(expression);
  if (match) {
    const result =
      match[2] === "+"
        ? addition(Number(match[1]), Number(match[3]))
        : subtract(Number(match[1]), Number(match[3]));
    return expression.replace(match[0], String(result));
  }
  return Number.NaN.toString();
};
