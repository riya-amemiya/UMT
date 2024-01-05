/**
 * 与えられた年が閏年かどうかを判定する
 * @param {number} year - 判定する年
 * @returns {boolean} 閏年であればtrue、そうでなければfalse
 */
export const isLeapYear = (year: number): boolean => {
  return (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
};
