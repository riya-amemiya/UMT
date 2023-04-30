import { newDateInt } from "@/Date";
import { now } from "@/Date/now";
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
	timeDifference = 9,
) => {
	const Bday = newDateInt(year, mon, day);
	const nowTime = now(timeDifference);
	const y = nowTime.getFullYear() - Bday.getFullYear();
	const r =
		nowTime < new Date(nowTime.getFullYear(), Bday.getMonth(), Bday.getDate())
			? y - 1
			: y;
	return year < 100 ? 1900 + y : r;
};
