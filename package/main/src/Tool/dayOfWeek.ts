import { newDateInt } from "@/Date/new";
import { now } from "@/Date/now";
import { dayTypeInt, monTypeInt } from "@/types/dateType";

export const dayOfWeek = <T extends monTypeInt,>(
	props?: {
		year?: number;
		mon?: T;
		day?: dayTypeInt<T>;
	},
	timeDifference = 9,
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
