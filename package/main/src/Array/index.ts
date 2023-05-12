import { arraysJoin } from "./arraysJoin";
import { getArraysCommon } from "./getArraysCommon";
import { getArraysDiff } from "./getArraysDiff";
import { quickSort } from "./quickSort";
import { sum } from "./sum";
import { map } from "./map";

export { arraysJoin, getArraysCommon, getArraysDiff, quickSort, sum, map };

export class UMTArrayClass {
	private localArraysJoin: typeof arraysJoin;
	private localGetArraysCommon: typeof getArraysCommon;
	private localGetArraysDiff: typeof getArraysDiff;
	private localQuickSort: typeof quickSort;
	private locaLsum: typeof sum;
	private localMap: typeof map;
	constructor() {
		this.localArraysJoin = arraysJoin;
		this.localGetArraysCommon = getArraysCommon;
		this.localGetArraysDiff = getArraysDiff;
		this.localQuickSort = quickSort;
		this.locaLsum = sum;
		this.localMap = map;
	}

	get arraysJoin() {
		return this.localArraysJoin;
	}
	get getArraysCommon() {
		return this.localGetArraysCommon;
	}
	get getArraysDiff() {
		return this.localGetArraysDiff;
	}
	get quickSort() {
		return this.localQuickSort;
	}
	get sum() {
		return this.locaLsum;
	}
	get map() {
		return this.localMap;
	}
}

export const UMT_Array = new UMTArrayClass();
