import { UMTSimpleDateClass } from "./Date/index";
import { UMTSimpleToolClass } from "./Tool";
export class UMTSimpleClass<LOCALDATE, LOCALTOOL> {
	private localDate: LOCALDATE;
	private localTool: LOCALTOOL;

	constructor(
		constructorLocalDateValue: LOCALDATE,
		constructorLocalToolValue: LOCALTOOL,
	) {
		this.localDate = constructorLocalDateValue;
		this.localTool = constructorLocalToolValue;
	}
	get getDate() {
		return this.localDate;
	}
	get getTool() {
		return this.localTool;
	}
}

export const UMT_Simple = new UMTSimpleClass(
	new UMTSimpleDateClass(),
	new UMTSimpleToolClass(),
);
