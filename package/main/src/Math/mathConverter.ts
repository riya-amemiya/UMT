import { mathSeparator } from "./mathSeparator";

/**
 * nの2乗を展開します
 * @param {string} equation - 変換する数学的な式
 * @returns {string} 変換後の式
 * @example mathConverter("1250*1250"); // "1500*1000+400*100+200*100+50*50"
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
