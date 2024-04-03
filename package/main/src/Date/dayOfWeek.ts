import type { hoursTypeInt } from "$/clockType";
import type { dayTypeInt, monTypeInt } from "$/dateType";
import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";

/**
 * 曜日を取得する
 * @param properties
 * @param timeDifference
 * @returns number
 * @example dayOfWeek({ year: 2000, mon: 1, day: 1 });
 */
export const dayOfWeek = <T extends monTypeInt>(
  properties?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference: hoursTypeInt = 9,
) => {
  const nowTime = now(timeDifference);
  if (properties) {
    return newDateInt(
      properties.year || nowTime.getFullYear(),
      properties.mon || ((nowTime.getMonth() + 1) as monTypeInt),
      properties.day || (nowTime.getDate() as dayTypeInt<T>),
    ).getDay();
  }
  return nowTime.getDay();
};
