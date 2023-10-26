import { ChunkArrayType } from "@/types/arrayType";

/**
 * 配列を指定した数で分割する
 * @param arr 配列
 * @param n 分割数
 * @returns 分割された配列
 */
export const chunk = <T extends unknown[], N extends number>(
  arr: T,
  n: N,
): ChunkArrayType<T, N> => {
  const result: T[][] = [];
  for (let i = 0; i < arr.length; i += n) {
    result.push(arr.slice(i, i + n) as unknown as T[]);
  }
  return result as ChunkArrayType<T, N>;
};
