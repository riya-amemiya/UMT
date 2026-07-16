/**
 * Shuffles all elements in a 2D array while maintaining the row lengths
 * @param array The 2D array to shuffle
 * @returns A new 2D array with shuffled elements
 * @example
 * shuffle2DArray([[1, 2], [3, 4], [5, 6]]);
 * // Result: [[1, 3], [6, 4], [2, 5]]
 */
export const shuffle2DArray = <T>(array: T[][]): T[][] => {
  const rowCount = array.length;
  let totalLength = 0;
  for (let row = 0; row < rowCount; row++) {
    totalLength += array[row].length;
  }

  const flatArray = new Array<T>(totalLength);
  let writeIndex = 0;
  for (let row = 0; row < rowCount; row++) {
    const subArray = array[row];
    const subLength = subArray.length;
    for (let column = 0; column < subLength; column++) {
      flatArray[writeIndex++] = subArray[column];
    }
  }

  for (let index = flatArray.length - 1; index > 0; index--) {
    const index_ = Math.floor(Math.random() * (index + 1));
    const temporary = flatArray[index];
    flatArray[index] = flatArray[index_];
    flatArray[index_] = temporary;
  }

  const result = new Array<T[]>(rowCount);
  let rowIndex = 0;
  for (let row = 0; row < rowCount; row++) {
    const rowLength = array[row].length;
    const newRow = new Array<T>(rowLength);
    for (let column = 0; column < rowLength; column++) {
      newRow[column] = flatArray[rowIndex++];
    }
    result[row] = newRow;
  }
  return result;
};
