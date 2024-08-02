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
 * 日付を生成する
 * @param year
 * @param mon
 * @param day
 * @param hours
 * @param minutes
 * @param seconds
 * @param milliseconds
 * @returns Date
 * @example newDateInt(2021, 1, 1); // 2021-01-01T00:00:00.000Z
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
 * 日付を生成する
 * @param date "2021-01-01"
 * @param hours
 * @param minutes
 * @param seconds
 * @param miliSeconds
 * @param timeDifference
 * @returns Date
 * @example newDateString("2021-01-01"); // 2021-01-01T00:00:00.000Z
 */
export const newDateString = <T extends MonTypeZero>(
  date: `${number}-${T}-${DayType<T>}`,
  hours: HoursType = "00",
  minutes: MinutesType = "00",
  seconds: SecondsType = "00",
  miliSeconds: MillisecondsType = "000",
  timeDifference: HoursType = "00",
): Date => {
  return new Date(
    `${date}T${hours}:${minutes}:${seconds}.${miliSeconds}+${timeDifference}:00`,
  );
};
