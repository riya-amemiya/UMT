import { mathSeparator } from "./mathSeparator";

/**
Expands square of n into a sum of simpler multiplications
@param {string} equation - Mathematical expression to convert
@returns {string} Converted expression
@example mathConverter("1250*1250"); // "1500*1000+400*100+200*100+50*50"
@description
This function converts expressions like n^2 or n*n into a sum of simpler multiplications
using the distributive property. For example, 1250² is converted to
(1000 + 200 + 50)² = 1500*1000 + 400*100 + 200*100 + 50*50.
An expansion that reproduces its own input, such as "5*5+0*0" for "5*5",
is returned after the first repetition instead of being expanded forever.
*/
export const mathConverter = (equation: string): string => {
  let convertedEquation = equation;

  while (true) {
    const multiplicationOrExponentiation =
      /\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?/.exec(convertedEquation);

    if (!multiplicationOrExponentiation) {
      return convertedEquation;
    }

    const [operand1, operator, operand2] =
      multiplicationOrExponentiation[0].split(/(\*|\^)/, 3);

    if (operand1 === operand2 || (operand2 && operator === "^")) {
      const [primary, remainder] = mathSeparator(operand1);

      if (!primary) {
        return convertedEquation;
      }

      const expanded = `${Number(operand1) + remainder}*${primary}+${
        remainder <= 100
          ? `${remainder}*${remainder}`
          : mathConverter(`${remainder}*${remainder}`)
      }`;

      if (expanded === convertedEquation) {
        return expanded;
      }

      convertedEquation = expanded;
    } else {
      return convertedEquation;
    }
  }
};
