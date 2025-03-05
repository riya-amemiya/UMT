/**
 * Repeatedly adds another string to the beginning of a string until it reaches the specified length.
 *
 * @param string_ - The original string to add padding to.
 * @param targetLength - The desired length of the string after padding.
 * @param padString - The string to add to the beginning of the original string.
 * @returns The padded string.
 * @throws {Error} If padString is empty.
 */
export const padStart = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  if (padString === "") {
    throw new Error("padString cannot be empty");
  }

  let padding = "";
  const paddingLength = targetLength - string_.length;
  while (padding.length < paddingLength) {
    padding += padString;
  }

  return padding.slice(0, paddingLength) + string_;
};
