import { dayType, dayTypeInt, monTypeInt, monTypeZero } from "@/types/dateType";
import {
	hoursType,
	hoursTypeInt,
	millisecondsTypeInt,
	minutesTypeInt,
	secondsTypeInt,
} from "@/types/clockType";

export const newDateInt = <T extends monTypeInt>(
	yer: number,
	mon: T,
	day: dayTypeInt<T>,
	hours: hoursTypeInt = 9,
	minutes: minutesTypeInt = 0,
	seconds: secondsTypeInt = 0,
	milliseconds: millisecondsTypeInt = 0,
): Date => {
	const date = new Date(
		yer,
		mon - 1,
		day,
		hours,
		minutes,
		seconds,
		milliseconds,
	);
	return date;
};
export const newDateStr = <T extends monTypeZero>(
	date: `${number}-${T}-${dayType<T extends monTypeZero ? T : never>}`,
	timeDifference: hoursType = "00",
): Date => {
	return new Date(`${date}T00:00:00+${timeDifference}:00`);
};
