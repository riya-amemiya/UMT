/**
 * 線形合同法による乱数生成器
 * @param seed 初期シード値
 * @param a 乗数 (デフォルト: 1664525)
 * @param c 加算定数 (デフォルト: 1013904223)
 * @param m モジュラス (デフォルト: 2^32)
 * @returns 乱数生成器関数
 * @example
 * const lcg = linearCongruentialGenerator(12345);
 */
export const linearCongruentialGenerator = (
  seed: number,
  a = 1_664_525,
  c = 1_013_904_223,
  m = 4_294_967_296, // 2 ** 32
) => {
  let current = seed;
  current = (a * current + c) % m;
  return current / m;
};
