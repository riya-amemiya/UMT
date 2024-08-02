import type { ZipArrayType } from "$/array/zip";

/**
 * 複数の配列を受け取り、それらの配列の要素を組み合わせて新しい配列を作成します。
 * @param {T} arrays - 組み合わせる配列のリスト
 * @returns {ZipArrayType<T>} - 各配列の要素が組み合わされた新しい配列
 */
export const zip = <T extends unknown[][]>(...arrays: T): ZipArrayType<T> => {
  if (arrays.length === 0) {
    return [] as unknown as ZipArrayType<T>;
  }
  const length = Math.min(...arrays.map((array) => array.length));
  return Array.from({ length }, (_, index) => {
    return arrays.map((array) => array[index]);
  }) as unknown as ZipArrayType<T>;
};
