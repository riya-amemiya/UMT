/**
 * 線形合同法による乱数生成器
 * @param seed 初期シード値
 * @param max 最大値 (デフォルト: 4294967296)
 * @param multiplier 乗数 (デフォルト: 1664525)
 * @param increment 加算定数 (デフォルト: 1013904223)
 * @returns 乱数生成器関数
 * @example
 * const lcg = linearCongruentialGenerator(12345);
 */
export const linearCongruentialGenerator = (
  seed: number,
  max = 4_294_967_296,
  multiplier = 1_664_525,
  increment = 1_013_904_223,
) => (multiplier * seed + increment) % max;
