/**
 * 重複をしないで結合
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
export const arraysJoin = <
    // rome-ignore lint/suspicious/noExplicitAny: <explanation>
    A extends any[],
    // rome-ignore lint/suspicious/noExplicitAny: <explanation>
    B extends any[],
    C extends A & B,
>(
    array: A,
    ...arrays: B[]
) => {
    if (!(array && Array.isArray(array))) {
        throw new Error('Invalid array');
    }

    for (const i of arrays) {
        if (!(i && Array.isArray(i))) {
            throw new Error('Invalid array');
        }

        array.push(...i);
    }

    return [...new Set(array)] as C;
};
