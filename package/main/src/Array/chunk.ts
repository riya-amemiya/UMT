import { ChunkArrayType } from "@/types/arrayType";

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
