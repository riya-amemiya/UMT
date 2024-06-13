import { padStart } from "@/String/padStart";
import { getTimezoneOffsetString } from "@/Date/getTimezoneOffsetString";

/**
 * 日時を指定されたフォーマットに従って文字列に変換します。
 * @param {Date} date - 対象の日時オブジェクト
 * @param {string} formatStr - フォーマット文字列
 * @return {string} - フォーマットされた日時文字列
 */
export const format = (
  date: Date,
  formatStr = "YYYY-MM-DDTHH:mm:ssZ",
): string => {
  if (!(date instanceof Date)) {
    throw new Error("Invalid Date in format");
  }

  const hours = date.getHours();
  const yearStr = String(date.getFullYear());
  const monthStr = String(date.getMonth() + 1);
  const dateStr = String(date.getDate());
  const hourStr = String(hours);
  const minuteStr = String(date.getMinutes());
  const secondStr = String(date.getSeconds());
  const millisecondStr = String(date.getMilliseconds());
  const dayStr = String(date.getDay());
  const ampm = hours < 12 ? "AM" : "PM";
  const timezoneOffsetStr = getTimezoneOffsetString(date);

  const matches: { [key: string]: string } = {
    YY: yearStr.slice(-2),
    YYYY: padStart(yearStr, 4, "0"),
    M: monthStr,
    MM: padStart(monthStr, 2, "0"),
    D: dateStr,
    DD: padStart(dateStr, 2, "0"),
    d: dayStr,
    H: hourStr,
    HH: padStart(hourStr, 2, "0"),
    h: String(hours % 12 || 12),
    hh: padStart(String(hours % 12 || 12), 2, "0"),
    a: ampm.toLowerCase(),
    A: ampm,
    m: minuteStr,
    mm: padStart(minuteStr, 2, "0"),
    s: secondStr,
    ss: padStart(secondStr, 2, "0"),
    SSS: padStart(millisecondStr, 3, "0"),
    Z: timezoneOffsetStr,
    ZZ: timezoneOffsetStr.replace(":", ""),
  };

  return formatStr.replace(
    /\[([^\]]+)]|Y{1,4}|M{1,2}|D{1,2}|d{1,2}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS/g,
    (match, $1) => $1 || matches[match],
  );
};
