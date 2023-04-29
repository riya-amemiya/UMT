import { now } from "./now";

export class DateWrapper extends Date {
	now: () => Date;
	constructor(date: Date) {
		super(date);
		this.now = now;
	}
}
