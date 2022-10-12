/**
 * 共通の要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
const getArraysCommon = (array: any[], ...arrays: any[]) => {
    const result: any[] = [];
    for (const i of array) {
        let flag = true;
        for (const j of arrays) {
            if (!j.includes(i)) {
                flag = false;
                break;
            }
        }
        if (flag) {
            result.push(i);
        }
    }
    return result;
};
export { getArraysCommon };
