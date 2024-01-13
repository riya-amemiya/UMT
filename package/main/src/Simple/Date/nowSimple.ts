import { hoursType, hoursTypeInt } from "$/clockType";
import { now } from "@/Date/now";
import { isNumber } from "@/Validate/isNumber";
/**
 * 現在時刻を取得する
 * @param timeDifference - 時差 (default: 9)
 * @returns Date
 * @example nowSimple(); // 2021-01-01T00:00:00.000Z
 */
export const nowSimple = (timeDifference: hoursTypeInt | hoursType = 9) => {
  if (typeof timeDifference === "number") {
    return now(timeDifference);
  }
  if (isNumber(timeDifference)) {
    return now(Number(timeDifference) as hoursTypeInt);
  }
  return now();
};
