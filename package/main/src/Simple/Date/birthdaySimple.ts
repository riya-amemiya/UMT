import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { DayType } from "$/date/dayType";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonType } from "$/date/monType";
import type { MonTypeInt } from "$/date/monTypeInt";
import { birthday } from "@/Date/birthday";
export type BIRTHDAYSIMPLE = <T extends MonType>(
  birthdays:
    | Date
    | `${number}-${T}-${DayType<T>}`
    | `${number}:${T}:${DayType<T>}`
    | `${number}/${T}/${DayType<T>}`
    | { year: number; mon: number; day: number },
  timeDifference?: HoursTypeInt,
) => number;
/**
 * Calculate age from birthdate
 * @param birthdays - Birthday date in various formats
 * @param timeDifference - Time zone difference in hours (default: 9)
 * @returns Age in years
 * @example birthdaySimple("2000-01-01");
 * birthdaySimple("2000:01:01");
 * birthdaySimple("2000/01/01");
 * birthdaySimple({ year: 2000, mon: 1, day: 1 });
 * birthdaySimple(new Date(2000, 0, 1));
 */
export const birthdaySimple = (<T extends MonType>(
  birthdays:
    | `${number}-${T}-${DayType<T>}`
    | `${number}:${T}:${DayType<T>}`
    | `${number}/${T}/${DayType<T>}`
    | Date
    | { year: number; mon: number; day: number },
  timeDifference: HoursTypeInt = 9,
) => {
  if (typeof birthdays === "string") {
    if (birthdays.includes(":")) {
      const [year, mon, day] = birthdays.split(":").map(Number) as [
        number,
        MonTypeInt,
        DayTypeInt<MonTypeInt>,
      ];
      return birthday(year, mon, day, timeDifference);
    }
    if (birthdays.includes("/")) {
      const [year, mon, day] = birthdays.split("/").map(Number) as [
        number,
        MonTypeInt,
        DayTypeInt<MonTypeInt>,
      ];
      return birthday(year, mon, day, timeDifference);
    }
    const [year, mon, day] = birthdays.split("-").map(Number) as [
      number,
      MonTypeInt,
      DayTypeInt<MonTypeInt>,
    ];
    return birthday(year, mon, day, timeDifference);
  }
  if (birthdays instanceof Date) {
    return birthday(
      birthdays.getFullYear(),
      birthdays.getMonth() as MonTypeInt,
      birthdays.getDate() as DayTypeInt<MonTypeInt>,
      timeDifference,
    );
  }
  return birthday(
    birthdays.year,
    birthdays.mon as MonTypeInt,
    birthdays.day as DayTypeInt<MonTypeInt>,
    timeDifference,
  );
}) as BIRTHDAYSIMPLE;
