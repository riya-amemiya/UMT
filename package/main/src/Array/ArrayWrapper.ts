import { map } from "./map";

export class ArrayWrapper {
	private readonly localMap: typeof map;
	// rome-ignore lint/suspicious/noExplicitAny: <explanation>
	private localData: any[] = [];
	constructor() {
		this.localMap = map;
		this.localData = [];
	}

	map<T, U>(
		array: T[] | Record<string, T>,
		callbackfn: (value: T, index: number, rowArray: typeof array) => U,
	) {
		this.localData = this.localMap(array, callbackfn);
		return this;
	}

	getData() {
		return this.localData;
	}
}
