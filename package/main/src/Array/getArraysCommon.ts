/**
 * 共通の要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */

// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const getArraysCommon = (array: any[], ...arrays: any[]) => {
    const result: unknown[] = [array, ...arrays].reduce(
        (prev, current) => {
            return prev.filter((item: unknown) =>
                current.includes(item),
            );
        },
    );
    return result;
    // const result: any[] = [];
    // for (const i of array) {
    //     let flag = true;
    //     for (const j of arrays) {
    //         if (!j.includes(i)) {
    //             flag = false;
    //             break;
    //         }
    //     }
    //     if (flag) {
    //         result.push(i);
    //     }
    // }
    // return result;
};
