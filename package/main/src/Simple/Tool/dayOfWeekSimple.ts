import { dayOfWeek } from "@/Tool/dayOfWeek";
import { hoursTypeInt } from "@/types/clockType";
import {
  MonthsWihout31Days,
  MonthsWihout31DaysInt,
  MonthsWith31Days,
  MonthsWith31DaysInt,
  convertMonTypeZero,
  dayType,
  dayTypeInt,
} from "@/types/dateType";

function dayOfWeekSimple<T extends MonthsWith31DaysInt | MonthsWihout31DaysInt>(
  props?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference?: hoursTypeInt,
): number;
function dayOfWeekSimple<T extends MonthsWith31Days | MonthsWihout31Days>(
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
    | MonthsWihout31Days
    | MonthsWith31DaysInt
    | MonthsWihout31DaysInt,
>(
  props?:
    | {
        year?: number;
        mon?: T;
        day?: T extends MonthsWith31Days | MonthsWihout31Days
          ? dayType<T>
          : T extends MonthsWith31DaysInt | MonthsWihout31DaysInt
          ? dayTypeInt<T>
          : never;
      }
    | `${number}-${T}-${T extends MonthsWith31Days | MonthsWihout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWihout31DaysInt
        ? dayTypeInt<T>
        : never}`
    | `${number}:${T}:${T extends MonthsWith31Days | MonthsWihout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWihout31DaysInt
        ? dayTypeInt<T>
        : never}`
    | `${number}/${T}/${T extends MonthsWith31Days | MonthsWihout31Days
        ? dayType<T>
        : T extends MonthsWith31DaysInt | MonthsWihout31DaysInt
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
    } else if (props.includes("/")) {
      const [year, mon, day] = props
        .split("/")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    } else {
      const [year, mon, day] = props
        .split("-")
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
  } else if (props instanceof Date) {
    return dayOfWeek(
      {
        year: props.getFullYear(),
        mon: (props.getMonth() + 1) as
          | MonthsWihout31DaysInt
          | MonthsWith31DaysInt,
        day: props.getDate() as dayTypeInt<
          MonthsWith31DaysInt | MonthsWihout31DaysInt
        >,
      },
      timeDifference,
    );
  } else {
    return dayOfWeek(
      props as {
        year?: number;
        mon?: MonthsWihout31DaysInt | MonthsWith31DaysInt;
        day?: dayTypeInt<MonthsWith31DaysInt | MonthsWihout31DaysInt>;
      },
      timeDifference,
    );
  }
}

export { dayOfWeekSimple };
