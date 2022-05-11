/**
 * 重複をしないで結合
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
const arraysJoin = (array: any[], ...arrays: any[]) => {
    for (const i of arrays) {
        array.push(...i);
    }
    return [...new Set(array)];
};
export default arraysJoin;
