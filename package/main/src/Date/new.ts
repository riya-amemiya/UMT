import type {
  hoursType,
  hoursTypeInt,
  millisecondsType,
  millisecondsTypeInt,
  minutesType,
  minutesTypeInt,
  secondsType,
  secondsTypeInt,
} from "$/clockType";
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
  hours: hoursTypeInt = (-new Date().getTimezoneOffset() / 60) as hoursTypeInt,
  minutes: minutesTypeInt = 0,
  seconds: secondsTypeInt = 0,
  milliseconds: millisecondsTypeInt = 0,
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
  hours: hoursType = "00",
  minutes: minutesType = "00",
  seconds: secondsType = "00",
  miliSeconds: millisecondsType = "000",
  timeDifference: hoursType = "00",
): Date => {
  return new Date(
    `${date}T${hours}:${minutes}:${seconds}.${miliSeconds}+${timeDifference}:00`,
  );
};
