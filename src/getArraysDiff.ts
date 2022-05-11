/**
 * 共通しない要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
const getArraysDiff = (array: any[], ...arrays: any[]) => {
    let hasArr: any[] = [];
    for (const i of arrays) {
        hasArr.push(...i);
    }
    const arr01 = [...new Set(array)];
    const arr02 = [...new Set(hasArr)];
    return [...arr01, ...arr02].filter(
        (value) => !arr01.includes(value) || !arr02.includes(value),
    );
};
export default getArraysDiff;
