/**
 * 小数点以下の桁数
 * @param  {number} value
 * @returns number
 * @example getDecimalLength(1.23); // 2
 */
export const getDecimalLength = (value: number) => {
  const x = `${value}`.split(".")[1];
  if (x !== undefined && x.length > 0) {
    return x.length;
  }
  return 0;
};
