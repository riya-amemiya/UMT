/**
 * 指定された要素数まで数値型の配列を生成する
 * @param length 配列の長さ
 * @param min 最小値 (デフォルト: 0)
 * @param max 最大値 (デフォルト: length - 1)
 * @param random ランダムな値を生成するかどうか (デフォルト: false)
 * @returns 数値型の配列
 * @example generateNumberArray(5); // [0, 1, 2, 3, 4]
 * @example generateNumberArray(5, 10, 14); // [10, 11, 12, 13, 14]
 */
export const generateNumberArray = (
  length: number,
  min = 0,
  max = length - 1,
  random = false,
): number[] => {
  if (length <= 0) {
    return [];
  }
  if (min > max) {
    throw new Error("min should be less than or equal to max");
  }
  if (length === 1) {
    return [min];
  }

  if (random) {
    return Array.from(
      { length },
      () => Math.floor(Math.random() * (max - min + 1)) + min,
    );
  }

  const step = (max - min) / (length - 1);
  return Array.from({ length }, (_, i) => min + i * step);
};
