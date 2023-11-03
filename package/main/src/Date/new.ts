import {
  hoursType,
  hoursTypeInt,
  millisecondsType,
  millisecondsTypeInt,
  minutesType,
  minutesTypeInt,
  secondsType,
  secondsTypeInt,
} from "@/types/clockType";
import { dayType, dayTypeInt, monTypeInt, monTypeZero } from "@/types/dateType";

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
export const newDateInt = <T extends monTypeInt>(
  year: number,
  mon: T,
  day: dayTypeInt<T>,
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
export const newDateString = <T extends monTypeZero>(
  date: `${number}-${T}-${dayType<T>}`,
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
