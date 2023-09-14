import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";
import { hoursTypeInt } from "@/types/clockType";
import { dayTypeInt, monTypeInt } from "@/types/dateType";

export const dayOfWeek = <T extends monTypeInt,>(
  props?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference: hoursTypeInt = 9,
) => {
  const nowTime = now(timeDifference);
  if (props) {
    return newDateInt(
      props.year || nowTime.getFullYear(),
      props.mon || ((nowTime.getMonth() + 1) as monTypeInt),
      props.day || (nowTime.getDate() as dayTypeInt<T>),
    ).getDay();
  } else {
    return nowTime.getDay();
  }
};
