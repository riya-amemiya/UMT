/**
 * 共通しない要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
const getArraysDiff = (array: any[], ...arrays: any[]): any[] => {
    const result: any[] = [];
    for (const i of array) {
        let flag = true;
        for (const j of arrays) {
            if (j.includes(i)) {
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
export { getArraysDiff };
