import { random } from "../Math/random";
/**
 * 配列を高速にソート
 * @param  {any[]} array
 * @param  {number} startID
 * @param  {number} endID
 */

// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const quickSort = <A extends any[]>(
  array: A,
  startID = 0,
  endID: number = array.length - 1,
) => {
<<<<<<< HEAD
  const pivot = array[random(endID, startID)];
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
=======
	const pivot = array[random(endID, startID)];
	let left = startID;
	let right = endID;
	let flag = true;
	while (flag) {
		while (array[left] < pivot) {
			left++;
		}
		while (pivot < array[right]) {
			right--;
		}
		if (right <= left) {
			flag = false;
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
>>>>>>> 58a3e53 (修正)
};
