/**
 * Shuffles all elements in a 2D array while maintaining the row lengths
 * @param array The 2D array to shuffle
 * @returns A new 2D array with shuffled elements
 * @example
 * shuffle2DArray([[1, 2], [3, 4], [5, 6]]);
 * // Result: [[1, 3], [6, 4], [2, 5]]
 */
export const shuffle2DArray = <T>(array: T[][]): T[][] => {
  const flatArray: T[] = [];
  const originalFlat: T[] = [];
  for (const subArray of array) {
    flatArray.push(...subArray);
    originalFlat.push(...subArray);
  }

  for (let index = flatArray.length - 1; index > 0; index--) {
    const index_ = Math.floor(Math.random() * (index + 1));
    [flatArray[index], flatArray[index_]] = [
      flatArray[index_],
      flatArray[index],
    ];
  }

  if (flatArray.length >= 2) {
    let isSame = true;
    for (let i = 0; i < originalFlat.length; i++) {
      if (flatArray[i] !== originalFlat[i]) {
        isSame = false;
        break;
      }
    }

    if (isSame) {
      [flatArray[0], flatArray[1]] = [flatArray[1], flatArray[0]];
    }
  }

  let rowIndex = 0;
  return array.map((subArray) => {
    const newRow = flatArray.slice(rowIndex, rowIndex + subArray.length);
    rowIndex += subArray.length;
    return newRow;
  });
};
