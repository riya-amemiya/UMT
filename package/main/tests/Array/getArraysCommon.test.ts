import { getArraysCommon } from "../../module/Array/getArraysCommon";
test("{getArraysCommon}", () => {
	expect(getArraysCommon([1, 2, 3], [2, 3, 4])).toEqual([2, 3]);
	expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5])).toEqual([3]);
	expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5], [3, 4, 5])).toEqual([
		3,
	]);
});
