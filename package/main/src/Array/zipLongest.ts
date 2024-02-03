import type { ZipArrayType } from "$/arrayType";

/**
 * 異なる長さの配列を組み合わせて、最も長い配列の長さに合わせた配列を生成します。
 * 短い配列の不足分は`undefined`で埋められます。
 *
 * @param {T} arrays - 組み合わせる配列のリスト。
 * @returns {ZipArrayType<T>} - 各配列の要素が組み合わされた新しい配列。
 *                             最も長い配列の長さに合わせて、短い配列の不足分は`undefined`で埋められます。
 */
export function zipLongest<T extends unknown[][]>(
  ...arrays: T
): ZipArrayType<T> {
  const maxLength = Math.max(...arrays.map((array) => array.length));
  return Array.from({ length: maxLength }, (_, index) => {
    return arrays.map((array) => array[index]);
  }) as unknown as ZipArrayType<T>;
}
