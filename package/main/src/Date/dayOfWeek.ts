import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonTypeInt } from "$/date/monTypeInt";
import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";

/**
 * 曜日を取得する
 * @param properties
 * @param timeDifference
 * @returns number
 * @example dayOfWeek({ year: 2000, mon: 1, day: 1 });
 */
export const dayOfWeek = <T extends MonTypeInt>(
  properties?: {
    year?: number;
    mon?: T;
    day?: DayTypeInt<T>;
  },
  timeDifference: HoursTypeInt = 9,
) => {
  const nowTime = now(timeDifference);
  if (properties) {
    return newDateInt(
      properties.year || nowTime.getFullYear(),
      properties.mon || ((nowTime.getMonth() + 1) as MonTypeInt),
      properties.day || (nowTime.getDate() as DayTypeInt<T>),
    ).getDay();
  }
  return nowTime.getDay();
};
