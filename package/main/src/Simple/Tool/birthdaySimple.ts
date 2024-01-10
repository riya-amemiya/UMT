import { hoursTypeInt } from "$/clockType";
import { dayType, dayTypeInt, monType, monTypeInt } from "$/dateType";
import { birthday } from "@/Tool/birthday";
export interface BIRTHDAYSIMPLE {
  <T extends monType>(
    birthdays:
      | Date
      | `${number}-${T}-${dayType<T>}`
      | `${number}:${T}:${dayType<T>}`
      | `${number}/${T}/${dayType<T>}`
      | { year: number; mon: number; day: number },
    timeDifference?: hoursTypeInt,
  ): number;
}
/**
 * 年齢を取得する
 * @param birthdays - 誕生日
 * @param timeDifference - 時差 (default: 9)
 * @returns number
 * @example birthdaySimple("2000-01-01");
 * birthdaySimple("2000:01:01");
 * birthdaySimple("2000/01/01");
 * birthdaySimple({ year: 2000, mon: 1, day: 1 });
 * birthdaySimple(new Date(2000, 0, 1));
 */
export const birthdaySimple = (<T extends monType>(
  birthdays:
    | `${number}-${T}-${dayType<T>}`
    | `${number}:${T}:${dayType<T>}`
    | `${number}/${T}/${dayType<T>}`
    | Date
    | { year: number; mon: number; day: number },
  timeDifference: hoursTypeInt = 9,
) => {
  if (typeof birthdays === "string") {
    if (birthdays.includes(":")) {
      const [year, mon, day] = birthdays.split(":").map(Number) as [
        number,
        monTypeInt,
        dayTypeInt<monTypeInt>,
      ];
      return birthday(year, mon, day, timeDifference);
    }
    if (birthdays.includes("/")) {
      const [year, mon, day] = birthdays.split("/").map(Number) as [
        number,
        monTypeInt,
        dayTypeInt<monTypeInt>,
      ];
      return birthday(year, mon, day, timeDifference);
    }
    const [year, mon, day] = birthdays.split("-").map(Number) as [
      number,
      monTypeInt,
      dayTypeInt<monTypeInt>,
    ];
    return birthday(year, mon, day, timeDifference);
  }
  if (birthdays instanceof Date) {
    return birthday(
      birthdays.getFullYear(),
      birthdays.getMonth() as monTypeInt,
      birthdays.getDate() as dayTypeInt<monTypeInt>,
      timeDifference,
    );
  }
  return birthday(
    birthdays.year,
    birthdays.mon as monTypeInt,
    birthdays.day as dayTypeInt<monTypeInt>,
    timeDifference,
  );
}) as BIRTHDAYSIMPLE;
