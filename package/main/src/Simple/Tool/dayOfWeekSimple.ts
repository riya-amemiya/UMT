import { dayOfWeek } from "@/Tool/dayOfWeek";
import {
  MonthsWith31Days,
  MonthsWihout31Days,
  dayType,
  MonthsWihout31DaysInt,
  MonthsWith31DaysInt,
  dayTypeInt,
} from "@/types/dateType";

export const dayOfWeekSimple = <
  T extends MonthsWith31Days | MonthsWihout31Days,
>(
  props?:
    | { year?: number; mon?: T; day?: dayType<T> }
    | `${number}-${T}-${dayType<T>}`
    | `${number}:${T}:${dayType<T>}`
    | `${number}/${T}/${dayType<T>}`
    | Date,
  timeDifference = 9,
) => {
  if (typeof props === "string") {
    if (props.includes(":")) {
      const [year, mon, day] = props
        .split(":")
        // rome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    } else if (props.includes("/")) {
      const [year, mon, day] = props
        .split("/")
        // rome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    } else {
      const [year, mon, day] = props
        .split("-")
        // rome-ignore lint/suspicious/noExplicitAny: <explanation>
        .map(Number) as any;
      return dayOfWeek({ year, mon, day }, timeDifference);
    }
  } else if (props instanceof Date) {
    return dayOfWeek(
      {
        year: props.getFullYear(),
        mon: props.getMonth() as MonthsWihout31DaysInt | MonthsWith31DaysInt,
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
};
