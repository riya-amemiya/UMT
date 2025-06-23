import { calculatorCore } from "./core";
import { literalExpression } from "./literalExpression";

/**
 * Calculator function that handles mathematical expressions and simple equations
 * Supports parentheses, signs, and currency conversion
 * Handles simple equations with single-character variables
 * @param {string} expression - Mathematical expression or equation
 * @param {object} exchange - Exchange rates for currency conversion
 * @returns {string} Calculation result
 * @example calculator("1+2"); // "3"
 * @example calculator("(2+3)*4"); // "20"
 * @example calculator("x=5"); // "x=5"
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
