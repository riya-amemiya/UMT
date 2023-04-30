import { birthdaySimple } from "./birthday";
import { dayOfWeekSimple } from "./dayOfWeekSimple";
import { deviationValueSimple } from "./deviationValueSimple";

export class UMTSimpleToolClass {
	private localBirthday: typeof birthdaySimple;
	private localDayOfWeek: typeof dayOfWeekSimple;
	private localDeviationValue: typeof deviationValueSimple;
	constructor() {
		this.localDayOfWeek = dayOfWeekSimple;
		this.localDeviationValue = deviationValueSimple;
		this.localBirthday = birthdaySimple;
	}
	get birthday() {
		return this.localBirthday;
	}
	get dayOfWeek() {
		return this.localDayOfWeek;
	}
	get deviationValue() {
		return this.localDeviationValue;
	}
}
