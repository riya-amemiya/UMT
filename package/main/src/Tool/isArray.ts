/**
 * 配列かどうかを判定する
 * @param array
 * @returns boolean
 * @example isArray([1, 2, 3]); // true
 * isArray({}); // false
 */
export const isArray = <T>(array: unknown): array is T[] => {
  return Array.isArray(array);
};
