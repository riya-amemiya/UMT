import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { ConvertMonTypeZero } from "$/date/convertMonTypeZero";
import type { DayType } from "$/date/dayType";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonthsWith31Days } from "$/date/monthsWith31Days";
import type { MonthsWith31DaysInt } from "$/date/monthsWith31DaysInt";
import type { MonthsWithout31Days } from "$/date/monthsWithout31Days";
import type { MonthsWithout31DaysInt } from "$/date/monthsWithout31DaysInt";
import { dayOfWeek } from "@/Date/dayOfWeek";
/**
 * 曜日を取得する
 * @param properties - 年月日
 * @param timeDifference - 時差 (default: 9)
 * @returns number
 * @example dayOfWeekSimple("2000-01-01");
 * dayOfWeekSimple("2000:01:01");
 * dayOfWeekSimple("2000/01/01");
 * dayOfWeekSimple({ year: 2000, mon: 1, day: 1 });
 * dayOfWeekSimple(new Date(2000, 0, 1));
 */
function dayOfWeekSimple<
  T extends MonthsWith31DaysInt | MonthsWithout31DaysInt,
>(
  properties?: {
    year?: number;
    mon?: T;
    day?: DayTypeInt<T>;
  },
  timeDifference?: HoursTypeInt,
): number;
function dayOfWeekSimple<T extends MonthsWith31Days | MonthsWithout31Days>(
  properties?:
    | `${number}-${ConvertMonTypeZero<T>}-${DayType<T>}`
    | `${number}:${ConvertMonTypeZero<T>}:${DayType<T>}`
    | `${number}/${ConvertMonTypeZero<T>}/${DayType<T>}`
    | Date,
  timeDifference?: HoursTypeInt,
): number;
function dayOfWeekSimple<
  T extends
    | MonthsWith31Days
    | MonthsWithout31Days
    | MonthsWith31DaysInt
    | MonthsWithout31DaysInt,
>(
  properties?:
    | {
        year?: number;
        mon?: T;
        day?: T extends MonthsWith31Days | MonthsWithout31Days
          ? DayType<T>
          : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
            ? DayTypeInt<T>
            : never;
      }
    | `${number}-${T}-${T extends MonthsWith31Days | MonthsWithout31Days
        ? DayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
          ? DayTypeInt<T>
          : never}`
    | `${number}:${T}:${T extends MonthsWith31Days | MonthsWithout31Days
        ? DayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
          ? DayTypeInt<T>
          : never}`
    | `${number}/${T}/${T extends MonthsWith31Days | MonthsWithout31Days
        ? DayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
          ? DayTypeInt<T>
          : never}`
    | Date,
  timeDifference: HoursTypeInt = 9,
): number {
  if (typeof properties === "string") {
    if (properties.includes(":")) {
      const [year, mon, day] = properties
        .split(":")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
    if (properties.includes("/")) {
      const [year, mon, day] = properties
        .split("/")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
    const [year, mon, day] = properties
      .split("-")
      // biome-ignore lint/suspicious/noExplicitAny: <explanation>
      .map(Number) as any;
    return dayOfWeek({ year, mon, day }, timeDifference);
  }
  if (properties instanceof Date) {
    return dayOfWeek(
      {
        year: properties.getFullYear(),
        mon: (properties.getMonth() + 1) as
          | MonthsWithout31DaysInt
          | MonthsWith31DaysInt,
        day: properties.getDate() as DayTypeInt<
          MonthsWith31DaysInt | MonthsWithout31DaysInt
        >,
      },
      timeDifference,
    );
  }
  return dayOfWeek(
    properties as {
      year?: number;
      mon?: MonthsWithout31DaysInt | MonthsWith31DaysInt;
      day?: DayTypeInt<MonthsWith31DaysInt | MonthsWithout31DaysInt>;
    },
    timeDifference,
  );
}

export { dayOfWeekSimple };
