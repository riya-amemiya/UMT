import { now } from "./now";

export class DateWrapper extends Date {
	now: () => Date;
	date: Date;
	year: number;
	month: number;
	day: number;
	constructor(date: Date = now()) {
		super(date);
		this.now = now;
		this.date = date;
		this.year = date.getFullYear();
		this.month = date.getMonth() + 1;
		this.day = date.getDate();
	}
	setNow(): Date {
		this.date = this.now();
		return this;
	}
	setYear(year: number): Date {
		this.date.setFullYear(year);
		return this;
	}
}

const a = new DateWrapper();
console.log(a.setNow().getDay());
