/**
 * 値がNaNかどうかを判定します。
 * @param value
 * @param loose 文字列も対象にするかどうか (default: false)
 * @returns boolean
 * @example isValueNaN(1); // false
 * isValueNaN("NaN"); // false
 * isValueNaN("NaN", true); // true
 */
export const isValueNaN = (value: unknown, loose = false): boolean => {
  // biome-ignore lint/suspicious/noGlobalIsNan: <explanation>
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
