import { isArr } from "@/Tool/isArr";

export const map = <T, U>(
	array: T[],
	callbackfn: (value: T, index: number, array: T[]) => U,
) => {
	const result = [];
	const isArray = isArr(array);
	if (isArray) {
		for (let i = 0; i < array.length; i++) {
			result.push(callbackfn(array[i], i, array));
		}
	} else {
		Object.values(array).forEach((value, index) => {
			result.push(callbackfn(value, index, array));
		});
	}
	return result;
};
