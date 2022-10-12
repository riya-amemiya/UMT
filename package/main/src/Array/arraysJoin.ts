/**
 * 重複をしないで結合
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
export const arraysJoin = <
    A extends any[],
    B extends any[],
    C extends A & B,
>(
    array: A,
    ...arrays: B[]
) => {
    for (const i of arrays) {
        array.push(...i);
    }
    return [...new Set(array)] as C;
};
