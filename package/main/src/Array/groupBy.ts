/**
 * 配列の要素をグループ化する
 * @param array グループ化する配列
 * @param iteratee 各要素に適用するグループ化関数
 * @returns グループ化されたオブジェクト
 */
export const groupBy = <T, K extends string | number>(
  array: T[],
  iteratee: (value: T, index: number) => K,
): Partial<Record<K, T[]>> => {
  return Object.groupBy(array, iteratee);
};
