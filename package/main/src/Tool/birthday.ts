import { now } from "@/Date/now";
/**
 * @param  {number} yer
 * @param  {number} mon
 * @param  {number} day
 * @param  {number} [timeDifference=9] 時差
 * @returns number 年齢
 */
export const birthday = (
	yer: number,
	mon: number,
	day: number,
	timeDifference = 9,
) => {
	const Bday = new Date(
		yer < 0 ? -yer : yer,
		mon < 0 ? -mon - 1 : mon - 1,
		day < 0 ? -day : day,
	);
	const nowTime = now(timeDifference);
	const y = nowTime.getFullYear() - Bday.getFullYear();
	const r =
		nowTime < new Date(nowTime.getFullYear(), Bday.getMonth(), Bday.getDate())
			? y - 1
			: y;
	return yer < 100 ? 1900 + y : r;
};
