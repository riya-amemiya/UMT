import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";
import { hoursTypeInt } from "@/types/clockType";
import { dayTypeInt, monTypeInt } from "@/types/dateType";

export const dayOfWeek = <T extends monTypeInt,>(
<<<<<<< HEAD
  props?: {
    year?: number;
    mon?: T;
    day?: dayTypeInt<T>;
  },
  timeDifference: hoursTypeInt = 9,
=======
	props?: {
		year?: number;
		mon?: T;
		day?: dayTypeInt<T>;
	},
	timeDifference: hoursTypeInt = 9,
>>>>>>> origin/main
) => {
  const nowTime = now(timeDifference);
  if (props) {
    return newDateInt(
      props.year || nowTime.getFullYear(),
      props.mon ? props.mon : (nowTime.getMonth() as monTypeInt),
      props.day || (nowTime.getDate() as dayTypeInt<T>),
    ).getDay();
  } else {
    return nowTime.getDay();
  }
};
