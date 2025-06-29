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
  const currentYear = nowTime.getFullYear();
  const birthYear = birthdayDate.getFullYear();

  // Calculate base age
  let age = currentYear - birthYear;

  // Check if birthday hasn't occurred this year yet
  const thisYearBirthday = new Date(
    currentYear,
    birthdayDate.getMonth(),
    birthdayDate.getDate(),
  );
  if (nowTime < thisYearBirthday) {
    age -= 1;
  }

  // Handle future birthdays (should return 0 or non-negative)
  if (age < 0) {
    return 0;
  }

  return age;
};
