import { DateWrapper, newDateInt } from "@/Date";
import { now } from "@/Date/now";
import { hoursTypeInt } from "@/types/clockType";
import { dayTypeInt, monTypeInt } from "@/types/dateType";
/**
 * @param  {number} year
 * @param  {number} mon
 * @param  {number} day
 * @param  {number} [timeDifference=9] 時差
 * @returns number 年齢
 */
export const birthday = <T extends monTypeInt>(
	year: number,
	mon: T,
	day: dayTypeInt<T>,
	timeDifference: hoursTypeInt = 9,
) => {
	const Bday = new DateWrapper(newDateInt(year, mon, day)).setTimeDifference(
		timeDifference,
	);
	const nowTime = now(timeDifference);
	const y = nowTime.getFullYear() - Bday.getYear();
	const r =
		nowTime < newDateInt(nowTime.getFullYear(), Bday.getMonth(), Bday.getDay())
			? y - 1
			: y;
	return year < 100 ? 1900 + y : r;
};
