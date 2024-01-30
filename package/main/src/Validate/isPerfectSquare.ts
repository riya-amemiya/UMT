/**
 * 与えられた数値が完全平方数かどうかを判定する
 * @param {number} number_ - 判定する数値
 * @returns {boolean} 完全平方数であればtrue、そうでなければfalse
 */
export const isPerfectSquare = (number_: number): boolean => {
  if (number_ < 0) {
    return false;
  }

  const sqrt = Math.sqrt(number_);
  return sqrt === Math.floor(sqrt);
};
