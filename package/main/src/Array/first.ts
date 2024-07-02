/**
 * 配列の先頭の要素を返す
 * @param array 配列
 * @returns 配列の先頭の要素
 * @example first([1, 2, 3]); // 1
 */
export const first = <T>(array: T[]): T | undefined => {
  return array.length > 0 ? array[0] : undefined;
};
