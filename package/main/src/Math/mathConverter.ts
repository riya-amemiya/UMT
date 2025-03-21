import { mathSeparator } from "./mathSeparator";

/**
 * Expands square of n into a sum of simpler multiplications
 * @param {string} equation - Mathematical expression to convert
 * @returns {string} Converted expression
 * @example mathConverter("1250*1250"); // "1500*1000+400*100+200*100+50*50"
 * @description
 * This function converts expressions like n^2 or n*n into a sum of simpler multiplications
 * using the distributive property. For example, 1250² is converted to
 * (1000 + 200 + 50)² = 1500*1000 + 400*100 + 200*100 + 50*50
 */
export const mathConverter = (equation: string): string => {
  let convertedEquation = equation;

  while (true) {
    const multiplicationOrExponentiation = convertedEquation.match(
      /\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/,
    );

    if (!multiplicationOrExponentiation) {
      return convertedEquation;
    }

    const [operand1, operator, operand2] =
      multiplicationOrExponentiation[0].split(/(\*|\^)/);

    if (operand1 === operand2 || (operand2 && operator === "^")) {
      const [primary, remainder] = mathSeparator(operand1);

      if (!primary) {
        return convertedEquation;
      }

      convertedEquation = `${Number(operand1) + remainder}*${primary}+`;

      convertedEquation +=
        remainder <= 100
          ? `${remainder}*${remainder}`
          : mathConverter(`${remainder}*${remainder}`);
    } else {
      return convertedEquation;
    }
  }
};
