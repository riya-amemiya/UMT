import { birthday } from "../../Tool/birthday";
import {
	MonthsWihout31Days,
	MonthsWith31Days,
	dayType,
} from "../../types/monType";
export interface BIRTHDAYSIMPLE {
	<T extends MonthsWith31Days | MonthsWihout31Days>(
		birthdays:
			| Date
			| `${number}-${T}-${dayType<T>}`
			| `${number}:${T}:${dayType<T>}`
			| `${number}/${T}/${dayType<T>}`
			| { yer: number; mon: number; day: number },
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
		| { yer: number; mon: number; day: number },
	timeDifference = 9,
) => {
	if (typeof birthdays === "string") {
		if (birthdays.includes(":")) {
			const [yer, mon, day] = birthdays.split(":").map(Number);
			return birthday(yer, mon, day, timeDifference);
		} else if (birthdays.includes("/")) {
			const [yer, mon, day] = birthdays.split("/").map(Number);
			return birthday(yer, mon, day, timeDifference);
		} else {
			const [yer, mon, day] = birthdays.split("-").map(Number);
			return birthday(yer, mon, day, timeDifference);
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
			birthdays.yer,
			birthdays.mon,
			birthdays.day,
			timeDifference,
		);
	}
}) as BIRTHDAYSIMPLE;
