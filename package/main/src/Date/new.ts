import type { HoursType } from "$/clock/hoursType";
import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { MillisecondsType } from "$/clock/millisecondsType";
import type { MillisecondsTypeInt } from "$/clock/millisecondsTypeInt";
import type { MinutesType } from "$/clock/minutesType";
import type { MinutesTypeInt } from "$/clock/minutesTypeInt";
import type { SecondsType } from "$/clock/secondsType";
import type { SecondsTypeInt } from "$/clock/secondsTypeInt";
import type { DayType } from "$/date/dayType";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonTypeInt } from "$/date/monTypeInt";
import type { MonTypeZero } from "$/date/monTypeZero";

/**
 * Create a new Date object from numeric values
 * @param year - The year
 * @param mon - The month (1-12)
 * @param day - The day of the month
 * @param hours - Hours offset from UTC (defaults to local timezone offset)
 * @param minutes - Minutes (0-59)
 * @param seconds - Seconds (0-59)
 * @param milliseconds - Milliseconds (0-999)
 * @returns Date object
 * @example newDateInt(2021, 1, 1); // Creates date for January 1, 2021
 */
export const newDateInt = <T extends MonTypeInt>(
  year: number,
  mon: T,
  day: DayTypeInt<T>,
  hours: HoursTypeInt = (-new Date().getTimezoneOffset() / 60) as HoursTypeInt,
  minutes: MinutesTypeInt = 0,
  seconds: SecondsTypeInt = 0,
  milliseconds: MillisecondsTypeInt = 0,
): Date => {
  const date = new Date(
    year,
    mon - 1,
    day,
    hours,
    minutes,
    seconds,
    milliseconds,
  );
  return date;
};

/**
 * Create a new Date object from a string date and time components
 * @param date - Date string in format "YYYY-MM-DD"
 * @param hours - Hours in "HH" format (00-23)
 * @param minutes - Minutes in "mm" format (00-59)
 * @param seconds - Seconds in "ss" format (00-59)
 * @param milliseconds - Milliseconds in "mmm" format (000-999)
 * @param timeDifference - Timezone offset in "HH" format (e.g., "09" for UTC+9)
 * @returns Date object
 * @example newDateString("2021-01-01"); // Creates date for January 1, 2021 00:00:00
 */
export const newDateString = <T extends MonTypeZero>(
  date: `${number}-${T}-${DayType<T>}`,
  hours: HoursType = "00",
  minutes: MinutesType = "00",
  seconds: SecondsType = "00",
  milliseconds: MillisecondsType = "000",
  timeDifference: HoursType = "00",
): Date => {
  return new Date(
    `${date}T${hours}:${minutes}:${seconds}.${milliseconds}+${timeDifference}:00`,
  );
};
