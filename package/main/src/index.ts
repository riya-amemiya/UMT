import { UMT_Array } from "./Array";
import { UMT_Date } from "./Date";
import { UMT_Math } from "./Math";
import { UMT_Object } from "./Object";
import { UMT_Simple } from "./Simple";
import { UMT_Tool } from "./Tool";
export class UMTClass {
	private loaclArray: typeof UMT_Array;
	private loaclDate: typeof UMT_Date;
	private loaclMath: typeof UMT_Math;
	private loaclSimple: typeof UMT_Simple;
	private loaclTool: typeof UMT_Tool;
	private loaclObject: typeof UMT_Object;
	constructor() {
		this.loaclArray = UMT_Array;
		this.loaclDate = UMT_Date;
		this.loaclMath = UMT_Math;
		this.loaclSimple = UMT_Simple;
		this.loaclTool = UMT_Tool;
		this.loaclObject = UMT_Object;
	}
	get getArray() {
		return this.loaclArray;
	}
	get getDate() {
		return this.loaclDate;
	}
	get getMath() {
		return this.loaclMath;
	}
	get getSimple() {
		return this.loaclSimple;
	}
	get getTool() {
		return this.loaclTool;
	}
	get getObject() {
		return this.loaclObject;
	}
}
export const UMT = new UMTClass();
