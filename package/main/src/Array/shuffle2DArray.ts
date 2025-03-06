/**
 * Shuffles all elements in a 2D array while maintaining the row lengths
 * @param array The 2D array to shuffle
 * @returns A new 2D array with shuffled elements
 * @example
 * shuffle2DArray([[1, 2], [3, 4], [5, 6]]);
 * // Result: [[1, 3], [6, 4], [2, 5]]
 */
export const shuffle2DArray = <T>(array: T[][]): T[][] => {
  // Flatten the 2D array into 1D and shuffle it
  const flatArray: T[] = [];
  for (const subArray of array) {
    flatArray.push(...subArray);
  }
  for (let index = flatArray.length - 1; index > 0; index--) {
    const index_ = Math.floor(Math.random() * (index + 1));
    [flatArray[index], flatArray[index_]] = [
      flatArray[index_],
      flatArray[index],
    ];
  }

  // Reconstruct the 2D array from the shuffled flat array
  let rowIndex = 0;
  return array.map((subArray) => {
    const newRow = flatArray.slice(rowIndex, rowIndex + subArray.length);
    rowIndex += subArray.length;
    return newRow;
  });
};
