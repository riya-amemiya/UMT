import type { TimeUnit } from "$/time/timeUnit";

/**
 * 時間単位間の変換率を定義
 */
const conversionRates: Record<TimeUnit, number> = {
  milliseconds: 1,
  seconds: 1000,
  minutes: 60 * 1000,
  hours: 60 * 60 * 1000,
};

/**
 * 時間を変換する関数
 * @param value 変換する値（文字列）
 * @param fromUnit 変換元の単位
 * @param toUnit 変換先の単位
 * @returns 変換後の値（数値）
 * @throws {Error} 無効な数値入力の場合
 */
export const convertTime = (
  value: string | number,
  fromUnit: TimeUnit,
  toUnit: TimeUnit,
): number => {
  const numericValue =
    typeof value === "string" ? Number.parseFloat(value) : value;

  const milliseconds = numericValue * conversionRates[fromUnit];
  return milliseconds / conversionRates[toUnit];
};
