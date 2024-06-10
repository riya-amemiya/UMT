import { padStart } from "@/String/padStart";

/**
 * タイムゾーンのオフセット文字列を取得します。
 * @param {Date} instance - 対象の日時オブジェクト
 * @return {string} - タイムゾーンのオフセット文字列
 */
export const getTimezoneOffsetString = (instance: Date): string => {
  const negMinutes = -instance.getTimezoneOffset();
  const minutes = Math.abs(negMinutes);
  const hourOffset = Math.floor(minutes / 60);
  const minuteOffset = minutes % 60;
  return `${negMinutes >= 0 ? "+" : "-"}${padStart(
    String(hourOffset),
    2,
    "0",
  )}:${padStart(String(minuteOffset), 2, "0")}`;
};
