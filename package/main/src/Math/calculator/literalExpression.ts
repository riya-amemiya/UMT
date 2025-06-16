import { calculatorCore } from "./core";

import { division } from "@/Math/division";
import { gcd } from "@/Math/gcd";
/**
 * Solves literal equations with variables
 * @param {string} x - Equation string
 * @returns {string} Solution result
 * @example literalExpression("x+1=2"); // "1"
 * @example literalExpression("2x=6"); // "3"
 * @example literalExpression("3x+2=8"); // "2"
 */
export const literalExpression = (x: string): string => {
  // Handle invalid equations like x=x
  const sides = x.split("=");
  if (sides.length === 2 && sides[0] === sides[1]) {
    return "";
  }

  // Store numerical and variable parts of the equation
  let numericalPart = "";
  let variablePart: string[] = [];

  // Split by equals sign and identify numerical and variable parts
  for (const part of sides) {
    if (/[A-Za-z]+/.test(part)) {
      variablePart = part
        .split(/(\d+[A-Za-z]+)|([^A-Za-z]+)/)
        .filter((n) => n && n !== undefined);
    } else {
      numericalPart = part;
    }
  }

  // Calculate the variable part (and invert signs for moving to other side of equation)
  if (variablePart[1]) {
    // Invert signs before calculating
    const invertedPart = variablePart[1]
      .replaceAll("+", "plus")
      .replaceAll("-", "minus")
      .replaceAll("plus", "-")
      .replaceAll("minus", "+");
    variablePart[1] = calculatorCore(invertedPart);
  }

  // Calculate the numerical part
  if (variablePart[1]) {
    // Ensure proper sign handling when combining
    const sign = variablePart[1].startsWith("-") ? "" : "+";
    numericalPart = calculatorCore(`${numericalPart}${sign}${variablePart[1]}`);
  } else {
    numericalPart = calculatorCore(numericalPart);
  }

  // Split the variable part again to separate coefficient and variable
  variablePart = variablePart[0]
    .split(/(\d+)|([A-Za-z]+)/)
    .filter((n) => n && n !== undefined);

  // If there's no coefficient, return the numerical result
  if (Number.isNaN(Number(variablePart[0]))) {
    return numericalPart;
  }

  // Simplify using greatest common divisor
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
