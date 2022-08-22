/**
 * 共通の要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
const getArraysCommon = <
    A extends any[],
    B extends any[],
    C extends A | B,
>(
    array: A,
    ...arrays: B[]
) => {
    let hasArr: any[] = [];
    for (const i of arrays) {
        hasArr.push(...i);
    }
    return [...new Set(array)].filter((value) =>
        hasArr.includes(value),
    ) as C;
};
export default getArraysCommon;
