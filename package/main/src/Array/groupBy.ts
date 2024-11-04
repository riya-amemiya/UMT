/**
 * 配列の要素をグループ化する
 * @param array グループ化する配列
 * @param iteratee 各要素に適用するグループ化関数
 * @returns グループ化されたオブジェクト
 */
export const groupBy = <T, K extends string | number>(
  array: T[],
  iteratee: (value: T, index: number, array: T[]) => K,
): Record<K, T[]> => {
  return array.reduce(
    (accumulator, value, index, array) => {
      const key = iteratee(value, index, array);
      if (!accumulator[key]) {
        accumulator[key] = [];
      }
      accumulator[key].push(value);
      return accumulator;
    },
    {} as Record<K, T[]>,
  );
};
