/**
 * Determines if a given integer is a perfect square
 * @param {number} number_ - Integer to check
 * @returns {boolean} true if the number is a perfect square, false otherwise
 * @example isPerfectSquare(16); // true
 * isPerfectSquare(25); // true
 * isPerfectSquare(10); // false
 * isPerfectSquare(-4); // false
 * isPerfectSquare(2.25); // false (non-integer input)
 */
export const isPerfectSquare = (number_: number): boolean => {
  if (number_ < 0) {
    return false;
  }

  const sqrt = Math.sqrt(number_);
  return sqrt === Math.floor(sqrt);
};
