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
  const array: number[] = [];

  // Handle invalid step
  if (step === 0) {
    return array;
  }

  const actualStart = end === undefined ? 0 : start;
  const actualEnd = end === undefined ? start : end;

  // Return empty array if invalid range
  if (
    (step > 0 && actualStart >= actualEnd) ||
    (step < 0 && actualStart <= actualEnd)
  ) {
    return array;
  }

  // Handle both positive and negative steps
  if (step > 0) {
    for (let index = actualStart; index < actualEnd; index += step) {
      // Handle floating point precision
      const roundedValue = Number(
        (Math.round(index * 1e10) / 1e10).toFixed(10),
      );
      array.push(roundedValue);
    }
  } else {
    for (let index = actualStart; index > actualEnd; index += step) {
      // Handle floating point precision
      const roundedValue = Number(
        (Math.round(index * 1e10) / 1e10).toFixed(10),
      );
      array.push(roundedValue);
    }
  }

  return array;
};
export { range };
