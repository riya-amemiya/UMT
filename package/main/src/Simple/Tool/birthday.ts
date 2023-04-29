import { birthday } from "@/Tool/birthday";
import { MonthsWihout31Days, MonthsWith31Days, dayType } from "$/dateType";
export interface BIRTHDAYSIMPLE {
	<T extends MonthsWith31Days | MonthsWihout31Days>(
		birthdays:
			| Date
			| `${number}-${T}-${dayType<T>}`
			| `${number}:${T}:${dayType<T>}`
			| `${number}/${T}/${dayType<T>}`
			| { year: number; mon: number; day: number },
		timeDifference?: number,
	): number;
}
export const birthdaySimple = (<
	T extends MonthsWith31Days | MonthsWihout31Days,
>(
	birthdays:
		| `${number}-${T}-${dayType<T>}`
		| `${number}:${T}:${dayType<T>}`
		| `${number}/${T}/${dayType<T>}`
		| Date
		| { year: number; mon: number; day: number },
	timeDifference = 9,
) => {
	if (typeof birthdays === "string") {
		if (birthdays.includes(":")) {
			const [year, mon, day] = birthdays.split(":").map(Number);
			return birthday(year, mon, day, timeDifference);
		} else if (birthdays.includes("/")) {
			const [year, mon, day] = birthdays.split("/").map(Number);
			return birthday(year, mon, day, timeDifference);
		} else {
			const [year, mon, day] = birthdays.split("-").map(Number);
			return birthday(year, mon, day, timeDifference);
		}
	} else if (birthdays instanceof Date) {
		return birthday(
			birthdays.getFullYear(),
			birthdays.getMonth(),
			birthdays.getDate(),
			timeDifference,
		);
	} else {
		return birthday(
			birthdays.year,
			birthdays.mon,
			birthdays.day,
			timeDifference,
		);
	}
}) as BIRTHDAYSIMPLE;
