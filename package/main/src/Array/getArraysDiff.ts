/**
 * 共通しない要素をとりだす
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 */
export const getArraysDiff = (
	array: unknown[],
	...arrays: unknown[]
): unknown[] => {
	const result = array.concat(...arrays).filter((val, _index, arr) => {
		return arr.indexOf(val) === arr.lastIndexOf(val);
	});
	return result;
};
