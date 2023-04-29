import { dayTypeInt, monTypeInt } from "@/types/dateType";
import { now } from "./now";
import {
	hoursTypeInt,
	millisecondsTypeInt,
	secondsTypeInt,
} from "@/types/clockType";

export class DateWrapper {
	now: (timeDifference: number) => Date;
	date: Date;
	timeDifference: number;
	/**
	 * @param date Date or number, numberを渡すとnow関数に渡される(UTCとの時差)
	 */
	constructor(date: Date | number = now()) {
		let tmp;
		if (typeof date === "number") {
			tmp = now(date);
		} else {
			tmp = date;
		}
		this.timeDifference = 9;
		this.now = now;
		this.date = tmp;
	}
	setDate(date: Date) {
		this.date = date;
		return this;
	}
	setNow() {
		this.date = this.now(this.timeDifference);
		return this;
	}
	setTimeDifference(timeDifference: number) {
		this.timeDifference = timeDifference;
		return this;
	}
	setYear(year: number) {
		this.date.setUTCFullYear(year);
		return this;
	}
	setMonth(month: monTypeInt) {
		this.date.setUTCMonth(month - 1);
		return this;
	}
	setDay(day: dayTypeInt<monTypeInt>) {
		this.date.setUTCDate(day);
		return this;
	}
	setHour(hour: hoursTypeInt) {
		this.date.setUTCHours(hour);
		return this;
	}
	setMinute(minute: millisecondsTypeInt) {
		this.date.setUTCMinutes(minute);
		return this;
	}
	setSecond(second: secondsTypeInt) {
		this.date.setUTCSeconds(second);
		return this;
	}
	setMillisecond(millisecond: millisecondsTypeInt) {
		this.date.setUTCMilliseconds(millisecond);
		return this;
	}
	getDateObj() {
		return {
			year: this.getYear(),
			month: this.getMonth(),
			day: this.getDay(),
			hour: this.getHour(),
			minute: this.getMinute(),
			second: this.getSecond(),
			millisecond: this.getMillisecond(),
			dayOfWeek: this.getDayOfWeek(),
		};
	}
	copyDate() {
		return Object.assign({}, this.date);
	}
	getDayOfWeek() {
		return this.date.getUTCDay();
	}
	getDate() {
		return this.date.getUTCDate();
	}
	getYear() {
		return this.date.getUTCFullYear();
	}
	getMonth() {
		return this.date.getUTCMonth() + 1;
	}
	getDay() {
		return this.date.getUTCDate();
	}
	getHour() {
		return this.date.getUTCHours();
	}
	getMinute() {
		return this.date.getUTCMinutes();
	}
	getSecond() {
		return this.date.getUTCSeconds();
	}
	getMillisecond() {
		return this.date.getUTCMilliseconds();
	}
	toString() {
		return this.date.toUTCString();
	}
	addYear(year: number) {
		this.date.setUTCFullYear(this.getYear() + year);
		return this;
	}
	addMonth(month: number) {
		this.date.setUTCMonth(this.getMonth() + month - 1);
		return this;
	}
	addDay(day: number) {
		this.date.setUTCDate(this.getDay() + day);
		return this;
	}
	addHour(hour: number) {
		this.date.setUTCHours(this.getHour() + hour);
		return this;
	}
	addMinute(minute: number) {
		this.date.setUTCMinutes(this.getMinute() + minute);
		return this;
	}
	addSecond(second: number) {
		this.date.setUTCSeconds(this.getSecond() + second);
		return this;
	}
	addMillisecond(millisecond: number) {
		this.date.setUTCMilliseconds(this.getMillisecond() + millisecond);
		return this;
	}
	subYear(year: number) {
		this.date.setUTCFullYear(this.getYear() - year);
		return this;
	}
	subMonth(month: number) {
		this.date.setUTCMonth(this.getMonth() - month - 1);
		return this;
	}
	subDay(day: number) {
		this.date.setUTCDate(this.getDay() - day);
		return this;
	}
	subHour(hour: number) {
		this.date.setUTCHours(this.getHour() - hour);
		return this;
	}
	subMinute(minute: number) {
		this.date.setUTCMinutes(this.getMinute() - minute);
		return this;
	}
	subSecond(second: number) {
		this.date.setUTCSeconds(this.getSecond() - second);
		return this;
	}
	subMillisecond(millisecond: number) {
		this.date.setUTCMilliseconds(this.getMillisecond() - millisecond);
		return this;
	}
}
