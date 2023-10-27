import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";
import { hoursTypeInt } from "@/types/clockType";
import { dayTypeInt, monTypeInt } from "@/types/dateType";

export const dayOfWeek = <T extends monTypeInt,>(
  properties?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference: hoursTypeInt = 9,
) => {
  const nowTime = now(timeDifference);
  if (properties) {
    return newDateInt(
      properties.year || nowTime.getFullYear(),
      properties.mon || ((nowTime.getMonth() + 1) as monTypeInt),
      properties.day || (nowTime.getDate() as dayTypeInt<T>),
    ).getDay();
  }
  return nowTime.getDay();
};
