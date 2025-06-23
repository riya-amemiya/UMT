/**
 * Adds the specified string to the end of the string until it reaches the specified length.
 *
 * @param string_ - The original string to apply padding
 * @param targetLength - The target length after padding
 * @param padString - The string to use for padding
 * @returns The string after padding has been applied
 */
export const padEnd = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  if (padString === "") {
    return string_;
  }
  let result = string_;
  while (result.length < targetLength) {
    result += padString.slice(0, targetLength - result.length);
  }
  return result;
};
