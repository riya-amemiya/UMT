import { bitwise } from "./bitwise";

/**
 * Xoshiro256** 乱数生成器
 * @param seed 初期シード値 (4つの32ビット整数の配列)
 * @returns 乱数生成器関数
 * @example
 * const xoshiro = xoshiro256ss([1, 2, 3, 4]);
 * console.log(xoshiro); // 乱数を生成
 */
export const xoshiro256 = (seed: [number, number, number, number]) => {
  let [s0, s1, s2, s3] = seed;
  const result = bitwise(s1 * 5, 7) * 9;

  const t = s1 << 9;

  s2 ^= s0;
  s3 ^= s1;
  s1 ^= s2;
  s0 ^= s3;

  s2 ^= t;

  s3 = bitwise(s3, 11);

  return (result >>> 0) / 2 ** 32;
};
