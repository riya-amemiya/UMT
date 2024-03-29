import type { hoursTypeInt } from "$/clockType";
import type { dayTypeInt, monTypeInt } from "$/dateType";
import { newDateInt } from "@/Date";
import { now } from "@/Date/now";
/**
 * @param  {number} year
 * @param  {number} mon
 * @param  {number} day
 * @param  {number} [timeDifference=9] 時差
 * @returns number 年齢
 * @example birthday(2000, 1, 1); // 21
 */
export const birthday = <T extends monTypeInt>(
  year: number,
  mon: T,
  day: dayTypeInt<T>,
  timeDifference: hoursTypeInt = 9,
) => {
  const birthdayDate = new Date(newDateInt(year, mon, day));
  const nowTime = now(timeDifference);
  const y = nowTime.getFullYear() - birthdayDate.getFullYear();
  const r =
    nowTime <
    newDateInt(
      nowTime.getFullYear(),
      (birthdayDate.getMonth() - 1) as monTypeInt,
      birthdayDate.getDay() as dayTypeInt<T>,
    )
      ? y - 1
      : y;
  return year < 100 ? 1900 + y : r;
};
