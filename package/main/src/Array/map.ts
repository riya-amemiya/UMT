import { isArr } from "@/Tool/isArr";

export const map = <T, U>(
	array: T[] | Record<string, T>,
	callbackfn: (value: T, index: number, rowArray: typeof array) => U,
): U[] => {
	const result: U[] = [];
	if (isArr(array)) {
		array.forEach((value, index, rowArray) => {
			result.push(callbackfn(value, index, rowArray));
		});
	} else {
		Object.keys(array).forEach((key, index) => {
			result.push(callbackfn(array[key], index, array));
		});
	}
	return result;
};
