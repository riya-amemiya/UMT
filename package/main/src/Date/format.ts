import { getTimezoneOffsetString } from "@/Date/getTimezoneOffsetString";
import { padStart } from "@/String/padStart";

/**
 * 日時を指定されたフォーマットに従って文字列に変換します。
 * @param {Date} date - 対象の日時オブジェクト
 * @param {string} formatString - フォーマット文字列
 * @return {string} - フォーマットされた日時文字列
 */
export const format = (
  date: Date,
  formatString = "YYYY-MM-DDTHH:mm:ssZ",
): string => {
  if (!(date instanceof Date)) {
    throw new TypeError("Invalid Date in format");
  }

  const hours = date.getHours();
  const yearString = String(date.getFullYear());
  const monthString = String(date.getMonth() + 1);
  const dateString = String(date.getDate());
  const hourString = String(hours);
  const minuteString = String(date.getMinutes());
  const secondString = String(date.getSeconds());
  const millisecondString = String(date.getMilliseconds());
  const dayString = String(date.getDay());
  const ampm = hours < 12 ? "AM" : "PM";
  const timezoneOffsetString = getTimezoneOffsetString(date);

  const matches: { [key: string]: string } = {
    YY: yearString.slice(-2),
    YYYY: padStart(yearString, 4, "0"),
    M: monthString,
    MM: padStart(monthString, 2, "0"),
    D: dateString,
    DD: padStart(dateString, 2, "0"),
    d: dayString,
    H: hourString,
    HH: padStart(hourString, 2, "0"),
    h: String(hours % 12 || 12),
    hh: padStart(String(hours % 12 || 12), 2, "0"),
    a: ampm.toLowerCase(),
    A: ampm,
    m: minuteString,
    mm: padStart(minuteString, 2, "0"),
    s: secondString,
    ss: padStart(secondString, 2, "0"),
    SSS: padStart(millisecondString, 3, "0"),
    Z: timezoneOffsetString,
    ZZ: timezoneOffsetString.replace(":", ""),
  };

  return formatString.replaceAll(
    /\[([^\]]+)]|(Y{1,4}|M{1,2}|D{1,2}|d{1,2}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS)/g,
    (_match, $1, $2) => $1 || matches[$2],
  );
};
