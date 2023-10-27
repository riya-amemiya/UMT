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
