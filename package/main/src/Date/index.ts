import { newDate } from "./new";
import { now } from "./now";

export { now, newDate };

export class UMTDateClass {
	private localNow: typeof now;
	private localNewDate: typeof newDate;
	constructor() {
		this.localNow = now;
		this.localNewDate = newDate;
	}
	get now() {
		return this.localNow;
	}
	get newDate() {
		return this.localNewDate;
	}
}

export const UMT_Date = new UMTDateClass();
