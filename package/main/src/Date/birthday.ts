import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonTypeInt } from "$/date/monTypeInt";
import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";
/**
 * Calculate age based on birthdate
 * @param  {number} year - Birth year
 * @param  {number} mon - Birth month (1-12)
 * @param  {number} day - Birth day
 * @param  {number} [timeDifference=9] - Time difference from UTC in hours
 * @returns {number} Age in years
 * @example birthday(2000, 1, 1); // Returns age of someone born on Jan 1, 2000
 */
export const birthday = <T extends MonTypeInt>(
  year: number,
  mon: T,
  day: DayTypeInt<T>,
  timeDifference: HoursTypeInt = 9,
) => {
  const birthdayDate = new Date(newDateInt(year, mon, day));
  const nowTime = now(timeDifference);
  const y = nowTime.getFullYear() - birthdayDate.getFullYear();
  const r =
    nowTime <
    newDateInt(
      nowTime.getFullYear(),
      (birthdayDate.getMonth() - 1) as MonTypeInt,
      birthdayDate.getDay() as DayTypeInt<T>,
    )
      ? y - 1
      : y;
  return year < 100 ? 1900 + y : r;
};
