import { hoursTypeInt } from "$/clockType";
import { OneHourMs } from "@/Consts/clock";

/**
 * タイムゾーンに関係なく、UTCを基準に指定した時差を加えた現在時刻を取得します。
 * @param {hoursTypeInt} [timeDifference=9] UTCからの時差を指定します。デフォルトは日本時間です。
 * @returns {Date} 現在時刻
 * @example now(); // Date
 */
export const now = (timeDifference: hoursTypeInt = 9): Date => {
  return new Date(Date.now() + timeDifference * OneHourMs);
};
