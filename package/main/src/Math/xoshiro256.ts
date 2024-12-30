import { bitwise } from "./bitwise";

/**
 * Xoshiro256**アルゴリズムによる乱数生成
 * @param state 4つの32ビット状態値の配列
 * @returns 生成された乱数値
 */
export const xoshiro256 = (
  state: [number, number, number, number],
  min = 0,
  max = 1,
): number => {
  const sum = (state[0] >>> 0) + (state[3] >>> 0);
  const result = bitwise(sum, 23) + (state[0] >>> 0);

  const t = state[1] << 17;

  state[2] ^= state[0];
  state[3] ^= state[1];
  state[1] ^= state[2];
  state[0] ^= state[3];

  state[2] ^= t;
  state[3] = bitwise(state[3], 45);

  return min + ((result >>> 0) / 2 ** 32) * (max - min);
};
