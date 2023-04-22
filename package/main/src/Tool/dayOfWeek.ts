import { now } from "../Date/now";
import {
	MonthsWihout31DaysInt,
	MonthsWith31DaysInt,
	dayTypeInt,
} from "../types/monType";

export const dayOfWeek = <
	T extends MonthsWith31DaysInt | MonthsWihout31DaysInt,
>(
	props?: {
		yer?: number;
		mon?: T;
		day?: dayTypeInt<T>;
	},
	timeDifference = 9,
) => {
	const nowTime = now(timeDifference);
	if (props) {
		return new Date(
			props.yer || nowTime.getFullYear(),
			props.mon ? props.mon - 1 : nowTime.getMonth(),
			props.day || nowTime.getDate(),
		).getDay();
	} else {
		return nowTime.getDay();
	}
};
