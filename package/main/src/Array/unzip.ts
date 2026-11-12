/**
Splits an array of tuples into one array per tuple position (the inverse of zip)
The number of result arrays is the shortest tuple length, matching zip's truncation.
@param {T[]} array Array of tuples to split
@returns {{ [K in keyof T]: T[K][] }} One array per tuple position
@example
unzip([[1, "a"], [2, "b"]]); // [[1, 2], ["a", "b"]]
unzip([[1, "a", true], [2, "b"]]); // [[1, 2], ["a", "b"]]
*/
export const unzip = <T extends unknown[]>(
  array: T[],
): { [K in keyof T]: T[K][] } => {
  const rowCount = array.length;
  if (rowCount === 0) {
    return [] as unknown as { [K in keyof T]: T[K][] };
  }

  let width = array[0].length;
  for (let row = 1; row < rowCount; row += 1) {
    if (array[row].length < width) {
      width = array[row].length;
    }
  }

  const result: unknown[][] = new Array(width);
  for (let position = 0; position < width; position += 1) {
    const column: unknown[] = new Array(rowCount);
    for (let row = 0; row < rowCount; row += 1) {
      column[row] = array[row][position];
    }
    result[position] = column;
  }

  return result as unknown as { [K in keyof T]: T[K][] };
};
