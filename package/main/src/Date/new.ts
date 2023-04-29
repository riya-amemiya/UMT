import { dayType, dayTypeInt, monTypeInt, monTypeZero } from "@/types/dateType";

export const newDateInt = <T extends monTypeInt>(
	yer: number,
	mon: T,
	day: dayTypeInt<T>,
): Date => {
	const date = new Date(yer, mon - 1, day);
	return date;
};

export const newDateStr = <T extends monTypeZero>(
	date: `${number}-${T}-${dayType<T>}`,
): Date => {
	return new Date(date);
};
