import { DateWrapper } from "./DateWrapper";
import { newDateInt, newDateStr } from "./new";
import { now } from "./now";

export { now, newDateInt, newDateStr, DateWrapper };

export class UMTDateClass {
	private localNow: typeof now;
	private localNewDateInt: typeof newDateInt;
	private localNewDateStr: typeof newDateStr;
	constructor() {
		this.localNow = now;
		this.localNewDateInt = newDateInt;
		this.localNewDateStr = newDateStr;
	}
	get now() {
		return this.localNow;
	}
	get newDateInt() {
		return this.localNewDateInt;
	}
	get newDateStr() {
		return this.localNewDateStr;
	}
}

export const UMT_Date = new UMTDateClass();
