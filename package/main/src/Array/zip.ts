import type { ZipArrayType } from "$/arrayType";

/**
 * 複数の配列を受け取り、それらの配列の要素を組み合わせて新しい配列を作成します。
 * @param {...Array<any>[]} arrays
 * @returns {Array<any[]>} 組み合わせた要素の新しい配列
 */
export function zip<T extends unknown[][]>(...arrays: T): ZipArrayType<T> {
  if (arrays.length === 0) {
    return [] as unknown as ZipArrayType<T>;
  }
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const length = Math.min(...arrays.map((array: any) => array.length));
  return Array.from({ length }, (_, index) => {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    return arrays.map((array: any) => array[index]);
  }) as unknown as ZipArrayType<T>;
}
