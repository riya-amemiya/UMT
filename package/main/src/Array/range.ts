/**
 * Generates an array of sequential numbers
 * @param start Starting number
 * @param end Ending number (if omitted, generates array from 0 to start)
 * @param step Step value (defaults to 1)
 * @returns Array of sequential numbers
 * @example range(5); // [0, 1, 2, 3, 4]
 * @example range(2, 10, 2); // [2, 4, 6, 8]
 */
const range = (start: number, end?: number, step = 1) => {
  if (step === 0) {
    return [];
  }

  const actualStart = end === undefined ? 0 : start;
  const actualEnd = end ?? start;

  if (
    (step > 0 && actualStart >= actualEnd) ||
    (step < 0 && actualStart <= actualEnd)
  ) {
    return [];
  }

  const estimatedLength = Math.ceil((actualEnd - actualStart) / step);
  const array = new Array<number>(Math.max(estimatedLength, 0));
  let writeIndex = 0;

  if (step > 0) {
    for (let index = actualStart; index < actualEnd; index += step) {
      array[writeIndex++] = Number(
        (Math.round(index * 1e10) / 1e10).toFixed(10),
      );
    }
  } else {
    for (let index = actualStart; index > actualEnd; index += step) {
      array[writeIndex++] = Number(
        (Math.round(index * 1e10) / 1e10).toFixed(10),
      );
    }
  }

  if (writeIndex !== array.length) {
    array.length = writeIndex;
  }

  return array;
};
export { range };
