import { ChunkArrayType } from "@/types/arrayType";

/**
 * 配列を指定した数で分割する
 * @param arr 配列
 * @param n 分割数
 * @returns 分割された配列
 */
export const chunk = <T extends unknown[], N extends number>(
  array: T,
  n: N,
): ChunkArrayType<T, N> => {
  const result: T[][] = [];
  for (let index = 0; index < array.length; index += n) {
    result.push(array.slice(index, index + n) as unknown as T[]);
  }
  return result as ChunkArrayType<T, N>;
};
