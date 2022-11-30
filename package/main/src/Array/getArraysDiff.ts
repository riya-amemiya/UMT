/**
 * 共通しない要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
export const getArraysDiff = (
    array: any[],
    ...arrays: any[]
): any[] => {
    const result = array
        .concat(...arrays)
        .filter((val, _index, arr) => {
            return arr.indexOf(val) === arr.lastIndexOf(val);
        });
    return result;
};
