/**
 * Performs multiplication without floating point errors for any number of arguments
 * @param  {...number[]} numbers Numbers to multiply
 * @returns {number} Product of all numbers
 * @example multiplication(0.1, 0.2, 0.3); // 0.006
 */
export const multiplication = (...numbers: number[]) => {
  let result = 1;
  let decimalPlaces = 0;

  for (const number of numbers) {
    if (number === 0) {
      // Short-circuit for zero
      return 0;
    }

    if (Number.isInteger(number)) {
      result *= number;
      continue;
    }

    const string_ = number.toString();
    const decimalIndex = string_.indexOf(".");

    if (decimalIndex === -1) {
      result *= number;
    } else {
      decimalPlaces += string_.length - decimalIndex - 1;
      result *= Number(string_.replace(".", ""));
    }
  }

  return result / 10 ** decimalPlaces;
};
