import { dayOfWeek } from "@/Tool/dayOfWeek";
import { hoursTypeInt } from "@/types/clockType";
import {
  MonthsWith31Days,
  MonthsWith31DaysInt,
  MonthsWithout31Days,
  MonthsWithout31DaysInt,
  convertMonTypeZero,
  dayType,
  dayTypeInt,
} from "@/types/dateType";

function dayOfWeekSimple<
  T extends MonthsWith31DaysInt | MonthsWithout31DaysInt,
>(
  props?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference?: hoursTypeInt,
): number;
function dayOfWeekSimple<T extends MonthsWith31Days | MonthsWithout31Days>(
  props?:
    | `${number}-${convertMonTypeZero<T>}-${dayType<T>}`
    | `${number}:${convertMonTypeZero<T>}:${dayType<T>}`
    | `${number}/${convertMonTypeZero<T>}/${dayType<T>}`
    | Date,
  timeDifference?: hoursTypeInt,
): number;
function dayOfWeekSimple<
  T extends
    | MonthsWith31Days
    | MonthsWithout31Days
    | MonthsWith31DaysInt
    | MonthsWithout31DaysInt,
>(
  props?:
    | {
        year?: number;
        mon?: T;
        day?: T extends MonthsWith31Days | MonthsWithout31Days
          ? dayType<T>
          : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
          ? dayTypeInt<T>
          : never;
      }
    | `${number}-${T}-${T extends MonthsWith31Days | MonthsWithout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
        ? dayTypeInt<T>
        : never}`
    | `${number}:${T}:${T extends MonthsWith31Days | MonthsWithout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
        ? dayTypeInt<T>
        : never}`
    | `${number}/${T}/${T extends MonthsWith31Days | MonthsWithout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWithout31DaysInt
        ? dayTypeInt<T>
        : never}`
    | Date,
  timeDifference: hoursTypeInt = 9,
): number {
  if (typeof props === "string") {
    if (props.includes(":")) {
      const [year, mon, day] = props
        .split(":")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
    if (props.includes("/")) {
      const [year, mon, day] = props
        .split("/")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
    const [year, mon, day] = props
      .split("-")
      // biome-ignore lint/suspicious/noExplicitAny: <explanation>
      .map(Number) as any;
    return dayOfWeek({ year, mon, day }, timeDifference);
  }
  if (props instanceof Date) {
    return dayOfWeek(
      {
        year: props.getFullYear(),
        mon: (props.getMonth() + 1) as
          | MonthsWithout31DaysInt
          | MonthsWith31DaysInt,
        day: props.getDate() as dayTypeInt<
          MonthsWith31DaysInt | MonthsWithout31DaysInt
        >,
      },
      timeDifference,
    );
  }
  return dayOfWeek(
    props as {
      year?: number;
      mon?: MonthsWithout31DaysInt | MonthsWith31DaysInt;
      day?: dayTypeInt<MonthsWith31DaysInt | MonthsWithout31DaysInt>;
    },
    timeDifference,
  );
}

export { dayOfWeekSimple };
