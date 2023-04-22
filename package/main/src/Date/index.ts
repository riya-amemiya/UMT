<<<<<<< HEAD
import { now } from './now';

export { now };

export class UMTDateClass {
    private localNow: typeof now;
    constructor() {
        this.localNow = now;
    }
    get now() {
        return this.localNow;
    }
=======
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
>>>>>>> adf67e3 (機能追加)
}

export const UMT_Date = new UMTDateClass();
