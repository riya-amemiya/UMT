/**
 * 配列を高速にソート
 * @param  {any[]} array
 * @param  {number} startID
 * @param  {number} endID
 */
const quickSort = <A extends any[]>(
    array: A,
    startID: number = 0,
    endID: number = array.length - 1,
) => {
    const pivot = array[Math.floor((startID + endID) / 2)];
    let left = startID;
    let right = endID;
    while (true) {
        while (array[left] < pivot) {
            left++;
        }
        while (pivot < array[right]) {
            right--;
        }
        if (right <= left) {
            break;
        }
        const tmp = array[left];
        array[left] = array[right];
        array[right] = tmp;
        left++;
        right--;
    }
    if (startID < left - 1) {
        quickSort(array, startID, left - 1);
    }
    if (right + 1 < endID) {
        quickSort(array, right + 1, endID);
    }
    return array;
};
export default quickSort;
