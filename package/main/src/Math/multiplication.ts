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
    if (number === 0) return 0;

    if (Number.isInteger(number)) {
      result *= number;
      continue;
    }

    const str = number.toString();
    const decimalIndex = str.indexOf(".");

    if (decimalIndex === -1) {
      result *= number;
    } else {
      decimalPlaces += str.length - decimalIndex - 1;
      result *= Number(str.replace(".", ""));
    }
  }

  return result / 10 ** decimalPlaces;
};
